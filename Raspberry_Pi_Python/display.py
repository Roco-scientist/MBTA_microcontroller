#!/usr/bin/env python
"""
Main python script for pulling and displaying MBTA arrival at Forest Hills Station
"""
import argparse
import datetime
import sys
import time
from typing import List

import requests

# import board
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
    mbta_api_site = f"https://api-v3.mbta.com/predictions?filter[stop]=place-{station}&filter[direction_id]={dir_code}&include=stop"
    predictions = requests.get(mbta_api_site).json()
    predictions_data = predictions['data']
    # Only take times that have a route id of cummuter rail
    commuter_rail_predictions = [prediction['attributes']['departure_time'] for prediction in predictions_data
                                 if prediction['relationships']['route']['data']['id'] == 'CR-Needham']
    # Convert the times to datetime format
    commuter_rail_times = [datetime.datetime.fromisoformat(
        dtime.replace("-04:00", "")) for dtime in commuter_rail_predictions]
    commuter_rail_times.sort()
    return commuter_rail_times


def display(times: List[datetime.datetime]) -> None:
    """
    TODO Fill this
    """
    # TODO fix below for the times
    # Clock display
    i2c = board.I2C()
    display = Seg7x4(i2c)
    display.brightness = 0.7
    now = datetime.datetime.now()
    next_train = times[0] - now
    display.print(next_train)

    # Screen display
    serial = spi()
    device = sh1106(serial)

    with canvas(device) as draw:
        draw.rectangle(device.bounding_box, outline="white", fill="black")
        draw.text((10, 10), f"Next train at {next_train}", fill="white")
        draw.text((10, 20), f"Second train at {second_train}", fill="white")
        draw.text((10, 30), f"Third train at {third_train}", fill="white")
    time.sleep(10)


def main():
    args = arguments()
    times = train_times(station=args.station, direction=args.direction, vehicle_type=args.type)
    display(times)


if __name__ == "__main__":
    if float(f"{sys.version_info[0]}.{sys.version_info[1]}") < 3.6:
        raise SystemExit("Python 3.6 or greater required")
    main()
