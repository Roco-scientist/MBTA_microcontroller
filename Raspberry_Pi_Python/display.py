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
    api_retrieval = requests.get(api).json()
    api_data = api_retrieval['data']
    commuter_rail_dep_time = {prediction['relationships']['trip']['data']['id']:
            datetime.datetime.fromisoformat(prediction['attributes']['departure_time'].replace("-04:00", "")) 
            for data in api_data}
#    commuter_rail_times = [datetime.datetime.fromisoformat(dtime.replace("-04:00", "")) 
#                           for dtime in commuter_rail_dep_time]
#    commuter_rail_times.sort()
    return commuter_rail_dep_time


def get_scheduled_times(station: str, dir_code: str, vehicle_type: str) -> Dict[str, datetime.datetime]:
    """
    TODO Fille this
    """
    now = datetime.now()
    schedules_api =
    f"https://api-v3.mbta.com/schedules?include=route,trip,stop&filter[min_time]={now.hour}%3A{now.minute}&filter[stop]=place-{station}&filter[route]=CR-Needham&filter[direction_id]={dir_code}"
    scheduled_times = get_routes_times(schedules_api)
    return scheduled_times


def get_prediction_times(station: str, dir_code: str, vehicle_type: str) -> Dict[str, datetime.datetime]:
    """
    TODO Fill this
    """
    predictions_api =
    f"https://api-v3.mbta.com/predictions?filter[stop]=place-{station}&filter[direction_id]={dir_code}&include=stop&filter[route]=CR-Needham"
    prediction_times = get_routes_times(predictions_api)
    return prediction_times


def train_times(station: str, direction: str, vehicle_type: str) -> List[datetime.datetime]:
    """
    TODO: Fill this
    """
    if direction == 'inbound':
        dir_code = 1
    elif direction == 'outbound':
        dir_code = 0
    else:
        raise SystemError(f"Direction not recognized: {direction}")
    scheduled_times = get_scheduled_times(stations, dir_code, vehicle_type)
    prediction_times = get_prediction_times(stations, dir_code, vehicle_type)
    for trip_id in prediction_times.keys():
        if trip_id not in list(scheduled_times.keys()):
            raise SystemError("Trip ID did not match.  Check code")
        scheduled_times[trip_id] = prediction_times[trip_id]
    commuter_rail_times = list(scheduled_times.values())
    commuter_rail_times.sort()
    return commuter_rail_times


def clear_clock() -> None:
    """
    TODO Fill this
    """
    i2c = board.I2C()
    display = Seg7x4(i2c)


def clock_countdown_time(train_time) -> Optional[str]:
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
            draw.text((5, 10), f"Next at {times[0].strftime('%H:%M')}", fill="white")
#            draw.text((10, 20), f"Second train at {second_train}", fill="white")
#            draw.text((10, 30), f"Third train at {third_train}", fill="white")
        train_countdown = clock_countdown_time(times[0])
        for _ in range(10):
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
