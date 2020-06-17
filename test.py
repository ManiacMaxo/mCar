#!/usr/bin/env python3

from time import sleep
from car import Car

car = Car()

print("forward left")
car.forward(0.5)
car.left(0.5)
sleep(2)
print('stop')
car.stop()
sleep(1)
print("backward right")
car.backward(1)
car.right(0.5)
sleep(2)
print('finish')
car.stop()
