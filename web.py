#!/usr/bin/env python3

import argparse
import logging
import os
import subprocess
import time

from bottle import default_app, route, static_file

from car import Car


@route('/control/<x:int>,<y:int>')
def action_control(x, y):
    car.drive(x / 100, y / 100)
    return [f'got x={x},y={y}']


@route('/')
def index():
    return static_file('index.html', root='www')


@route('/joy.min.js')
def index():
    return static_file('joy.min.js', root='www')


if __name__ == '__main__':

    parser = argparse.ArgumentParser()

    # Server settings
    parser.add_argument(
        '-i', '--host', default=os.getenv('IP', '0.0.0.0'), help='IP Address')
    parser.add_argument(
        '-p', '--port', default=os.getenv('PORT', 80), help='Port')

    # Verbose mode
    parser.add_argument('--verbose', '-v',
                        help='increase output verbosity', action='store_true')
    args = parser.parse_args()

    if args.verbose:
        logging.basicConfig(level=logging.DEBUG)
    else:
        logging.basicConfig(level=logging.ERROR)
    log = logging.getLogger(__name__)

    try:
        car = Car()
        car.stop()
    except Exception as e:
        log.error(e)
        exit()

    try:
        app = default_app()
        app.run(host=args.host, port=args.port, server='tornado')
    except:
        log.error(f'Unable to start server on {args.host}:{args.port}')
