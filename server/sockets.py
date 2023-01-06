
from utils import car
from .server import socket_manager as sm

@sm.on('connect')
def connection(sid, *args, **kwargs):
    print(f"connection established {sid}")
    return "acknowledge"

@sm.on('control')
def control(sid, x, y, *args, **kwargs):
    print(f"control event x: {x}, y: {y}")
    car.drive(x, y)

@sm.on('stop')
def stop(sid, *args, **kwargs):
    print("stop event")
    car.stop()


