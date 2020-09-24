#!/usr/bin/env python
"""
Main python script for pulling and displaying MBTA arrival at Forest Hills Station
"""
import argparse
import datetime
import sys
import time
import concurrent.futures
from typing import List, Optional, Dict

import requests

import board
from adafruit_ht16k33.segments import Seg7x4
from luma.core.interface.serial import spi
from luma.core.render import canvas
from luma.oled.device import sh1106, ssd1306


__author__ = "Rory Coffey"
__email__ = "coffeyrt@gmail.com"
__date__ = "2020-09-21"


def arguments():
    parser = argparse.ArgumentParser(
        description="Get and display station commuter rail arrival times")
    parser.add_argument("-s", dest="station", type=str, default='forhl',
                        help="MBTA station abbreviation [default: forhl]")
    parser.add_argument("-d", dest="direction", type=str, default='inbound',
                        choices=('outbound', 'inbound'), help="MBTA direction")
    parser.add_argument("-t", dest="type", type=str, default='commuter_rail',
                        choices=('commuter_rail', 'subway', 'bus'),
                        help="Type of transportation (not working/implemented yet)")
    return parser.parse_args()


def get_routes_times(api: str) -> Dict[str, datetime.datetime]:
    """
    Retrieves the route times from the MBTA API.  Returns a dictionary of times in order to combine scheduled
    and predicted times.
    :api: website string corresponding to the MBTA API pull to get a JSON of results
    :return: dictionary with the route id as key and the time as value
    """
    # Retrieve the MBTA data JSON
    api_retrieval = requests.get(api).json()
    # If there is any data proceed
    if len(api_retrieval) > 0 and 'data' in list(api_retrieval.keys()):
        # Pull data child node.  There are a couple of other nodes not wanted
        api_data = api_retrieval['data']
        # Create dictionary of {trip id: departure time}
        commuter_rail_dep_time = {}
        # Iterate through routes
        for data in api_data:
            # Pull the departure time and remove the timezone information
            departure_time = data['attributes']['departure_time'].replace("-04:00", "") 
            # Convert to datetime.datetime
            departure_time_datetime = datetime.datetime.fromisoformat(departure_time)
            # Pull the trip id
            trip_id = data['relationships']['trip']['data']['id']
            # Add to dictionary
            commuter_rail_dep_time[trip_id] = departure_time_datetime
        return commuter_rail_dep_time
    # return empty dictionary if there is no data.  This is common with predictions if there isn't
    # one soon enough
    return dict()


def get_scheduled_times(station: str, dir_code: str, vehicle_type: str) -> Dict[str, datetime.datetime]:
    """
    Creates the API for MBTA scheduled times
    :station: station of interest
    :dir_code: dir code for MBTA API. 0 for outbound, 1 for inbound
    :vehicle_type: subway, bus, commuter rail.  Not yet integrated
    :return: dictionary of {trip id: departure time}
    """
    # Now in datetime format to pull only later schedules.  Not working.  Need to fix
    now = datetime.datetime.now()
    # The MBTA schedules api with variable added
    schedules_api = \
    f"https://api-v3.mbta.com/schedules?include=route,trip,stop&filter[min_time]={now.hour}%3A{now.minute}&filter[stop]=place-{station}&filter[route]=CR-Needham&filter[direction_id]={dir_code}"
    # Gets the route times dictionary
    scheduled_times = get_routes_times(schedules_api)
    return scheduled_times


def get_prediction_times(station: str, dir_code: str, vehicle_type: str) -> Dict[str, datetime.datetime]:
    """
    Creates the API for MBTA prediction times. If the pull happens outside of prediction windows, an
    empty dictionary is returned
    :station: station of interest
    :dir_code: dir code for MBTA API. 0 for outbound, 1 for inbound
    :vehicle_type: subway, bus, commuter rail.  Not yet integrated
    :return: dictionary of {trip id: departure time}
    """
    # The MBTA prediction api with variables added
    predictions_api = \
    f"https://api-v3.mbta.com/predictions?filter[stop]=place-{station}&filter[direction_id]={dir_code}&include=stop&filter[route]=CR-Needham"
    # Gets the route times dictionary.  May be empty if there isn't one departing soon enough
    prediction_times = get_routes_times(predictions_api)
    return prediction_times


def train_times(station: str, direction: str, vehicle_type: str) -> List[datetime.datetime]:
    """
    Retrieves the departure times of the wanted MBTA vehicle. Also replaces scheduled times with
    prediction times, if they exists.
    :station: desired station abbreviation.  Can be found within the HTML website at MBTA
    :direction: inbound or outbound
    :vehicle_type: subway, bus, commuter rail.  Not yet integrated
    :return: a list of train/bus departure times
    """
    # Convert direction to direction code
    dir_code_conversion = {'inbound': 1, 'outbound': 0}
    # Get scheduled times
    scheduled_times = get_scheduled_times(station, dir_code_conversion[direction], vehicle_type)
    # Get prediction times
    prediction_times = get_prediction_times(station, dir_code_conversion[direction], vehicle_type)
    # Replace scheduled times with prediction times by trip id
    for trip_id in prediction_times.keys():
        if trip_id not in list(scheduled_times.keys()):
            raise SystemError("Trip ID did not match.  Check code")
        scheduled_times[trip_id] = prediction_times[trip_id]
    # Get a list of all departure times
    commuter_rail_times = list(scheduled_times.values())
    now = datetime.datetime.now()
    # API did not work for only retrieving later times so this is a quick fix to do so
    commuter_rail_times = [time for time in commuter_rail_times if time > now]
    # Sort the times so the nearest time is first
    commuter_rail_times.sort()
    return commuter_rail_times


def clear_clock() -> None:
    """
    Clears the clock display.  Otherwise it stays on, even after the program runs
    """
    # Use I2C connection for clock display
    i2c = board.I2C()
    # Create a new display which clears the current display
    display = Seg7x4(i2c)


def clock_countdown_time(train_times: List[datetime.datetime], min_clock_display: int) -> Optional[str]:
    """
    Takes in the train departure times and finds the difference between the soonest departure and
    now. Aslo has a minimum difference varaible so that if it takes a certain amount of time to walk
    to station, to account for that and only display a countdown that is achievable to walk to.
    :train_times: list of train departure times
    :min_clock_display: the minimum difference to display in minutes
    :return: Either the display countdown time or None if there isn't one within 99 minutes, the
            display limit
    """
    # Retrieve current time to compare
    now = datetime.datetime.now()
    # Get the difference between the soonest train and now
    train_num = 0
    next_train = train_times[train_num] - now
    # get the minutes for the display
    next_train_minutes = next_train.seconds / 60
    # While the difference is less than the minimum, use the next train/bus
    while (next_train_minutes < min_clock_display) and train_num < len(train_times):
        train_num += 1
        next_train = train_times[train_num] - now
        next_train_minutes = next_train.seconds / 60
    # get seconds for the display
    next_train_seconds = str(next_train.seconds % 60)
    # make sure it is 0 padded
    if len(next_train_seconds) == 1:
        next_train_seconds = f"0{next_train_seconds}"
    # make sure it is 0 padded
    next_train_minutes = str(int(next_train_minutes))
    if len(next_train_minutes) == 1:
        next_train_minutes = f"0{next_train_seconds}"
    # If minutes are not a length of 3 (too large for display)
    if len(next_train_minutes) == 2 and int(next_train_minutes) > min_clock_display:
        # Return the display text
        return f"{next_train_minutes}:{next_train_seconds}"
    # Else return None
    return None


def display(times: List[datetime.datetime], screen_display,
        clock_display, min_clock_display: int = 0) -> None:
    """
    Pulls in times and pushes the information to the clock and screen display
    :times: list of departure times
    :min_clock_display: the minimum difference in minutes to display on the countdown clock
    """
    # If there are departure times, continue
    if len(times) > 0:
        # Display times on the SPI sh1106 screen
        with canvas(screen_display) as draw:
            draw.rectangle(screen_display.bounding_box, outline="white", fill="black")
            draw.text((25, 10), "Schedule: ", fill="white")
            # For up to 3 train times, display their schedule
            for x in range(min(3, len(times))):
                train_num = x + 1
                display_y = (train_num * 10) + 10
                # Display train number and departure time
                draw.text((25, display_y), f"{train_num}: {times[x].strftime('%H:%M')}", fill="white")
        # Update coutdown clock every half second
        for _ in range(10):
            # Get the countdown display time
            train_countdown = clock_countdown_time(times, min_clock_display)
            # If a time is returned, display
            if train_countdown is not None:
                clock_display.print(train_countdown)
            # Else clear the clock
            else:
                clear_clock()
            # Pause for a half second
            time.sleep(0.5)
    else:
        # If there are no predicted departures, display on screen
        with canvas(screen_display) as draw:
            draw.rectangle(screen_display.bounding_box, outline="white", fill="black")
            draw.text((5, 30), "No Predicted Arrivals", fill="white")
        # Pause for 5 seconds to not pull too often from API.  Same paus is achieved with countdown
        # clock above
        time.sleep(5)


def main() -> None:
    # retrieve arguments
    script_args = arguments()
    # loop for retrieving new times and display.  Will try to adjust in the future to kill with a
    # different thread
    # setup clock display
    i2c = board.I2C()
    clock_display = Seg7x4(i2c)
    clock_display.brightness = 0.4
    # Setup screen display
    serial = spi()
    screen_display = sh1106(serial)
    # get departure times
    times = train_times(station=script_args.station, direction=script_args.direction, vehicle_type=script_args.type)
    # display departure times and countdown on screen and clock
    display(times, screen_display, clock_display)
    for _ in range(10):
        # get departure times
        executor = concurrent.futures.ThreadPoolExecutor()
        future = executor.submit(train_times, script_args.station, script_args.direction, script_args.type)
        # display departure times and countdown on screen and clock
        display(times, screen_display, clock_display)
        times = future.result()
        executor.shutdown(wait=False)
    # clear clock when done
    clear_clock()


if __name__ == "__main__":
    if float(f"{sys.version_info[0]}.{sys.version_info[1]}") < 3.6:
        raise SystemExit("Python 3.6 or greater required")
    main()
