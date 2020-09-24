#!/usr/bin/env python
"""
Main python script for pulling and displaying MBTA arrival at Forest Hills Station
"""
import argparse
import datetime
import sys
import time
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
    Retrieves the route times from the MBTA API.  Return a dictionary of times to combine scheduled
    and predicted times.
    :api: website string corresponding to the MBTA API pull to get a JSON of results
    :return: dictionary with the route id as key and the time as value
    """
    # Retrieve the MBTA data JSON
    api_retrieval = requests.get(api).json()
    # If there is any data proceed
    if len(api_retrieval) > 0:
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
    :return:
    """
    if direction == 'inbound':
        dir_code = 1
    elif direction == 'outbound':
        dir_code = 0
    else:
        raise SystemError(f"Direction not recognized: {direction}")
    scheduled_times = get_scheduled_times(station, dir_code, vehicle_type)
    prediction_times = get_prediction_times(station, dir_code, vehicle_type)
    for trip_id in prediction_times.keys():
        if trip_id not in list(scheduled_times.keys()):
            raise SystemError("Trip ID did not match.  Check code")
        scheduled_times[trip_id] = prediction_times[trip_id]
    commuter_rail_times = list(scheduled_times.values())
    now = datetime.datetime.now()
    commuter_rail_times = [time for time in commuter_rail_times if time > now]
    commuter_rail_times.sort()
    return commuter_rail_times


def clear_clock() -> None:
    """
    TODO Fill this
    """
    i2c = board.I2C()
    display = Seg7x4(i2c)


def clock_countdown_time(train_time, min_clock_display) -> Optional[str]:
    """
    TODO Fill this
    """
    now = datetime.datetime.now()
    next_train = train_time - now
    next_train_seconds = str(next_train.seconds % 60)
    if len(next_train_seconds) == 1:
        next_train_seconds = f"0{next_train_seconds}"
    next_train_minutes = str(int(next_train.seconds / 60))
    if len(next_train_minutes) == 1:
        next_train_minutes = f"0{next_train_seconds}"
    if len(next_train_minutes) == 2 and int(next_train_minutes) > min_clock_display:
        return f"{next_train_minutes}:{next_train_seconds}"
    return None


def display(times: List[datetime.datetime], min_clock_display: int = 0) -> None:
    """
    TODO Fill this
    """
    # setup clock display
    i2c = board.I2C()
    display = Seg7x4(i2c)
    display.brightness = 0.7
    # Setup screen display
    serial = spi()
    device = sh1106(serial)
    if len(times) > 0:
        with canvas(device) as draw:
            draw.rectangle(device.bounding_box, outline="white", fill="black")
            draw.text((5, 10), "Schedule: ", fill="white")
            for x in range(min(3, len(times))):
                train_num = x + 1
                display_y = (train_num * 10) + 10
                draw.text((5, display_y), f"{train_num}: {times[x].strftime('%H:%M')}", fill="white")
        for _ in range(10):
            train_countdown = clock_countdown_time(times[0], min_clock_display)
            if train_countdown is not None:
                display.print(train_countdown)
            else:
                clear_clock()
            time.sleep(0.5)
    else:
        with canvas(device) as draw:
            draw.rectangle(device.bounding_box, outline="white", fill="black")
            draw.text((5, 10), "No Predicted Arrivals", fill="white")
        time.sleep(5)


def main():
    args = arguments()
    for _ in range(5):
        times = train_times(station=args.station, direction=args.direction, vehicle_type=args.type)
        display(times)
    clear_clock()


if __name__ == "__main__":
    if float(f"{sys.version_info[0]}.{sys.version_info[1]}") < 3.6:
        raise SystemExit("Python 3.6 or greater required")
    main()
