
import time

from gpiozero import Motor

motor = Motor(9, 10)
# turn = Motor(7, 8)

motor.forward()
