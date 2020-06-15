#! bin/usr/env/python3
from time import sleep

from gpiozero import DigitalOutputDevice, PWMOutputDevice


class Car:
    def __init__(self):
        self.forw = DigitalOutputDevice(9)
        self.back = DigitalOutputDevice(10)
        self.left = DigitalOutputDevice(7)
        self.right = DigitalOutputDevice(8)

    def forward(self, time=1):
        self.back.off()
        # assure that both pins aren't on at the same time
        self.forw.on()
        sleep(time)
        self.forw.off()

    def backward(self, time=1):
        self.forw.off()
        # assure that both pins aren't on at the same time
        self.back.on()
        sleep(time)
        self.back.off()

    def left(self, time=1):
        self.right.off()
        # assure that both pins aren't on at the same time
        self.left.on()
        sleep(time)
        self.left.off()

    def right(self, time=1):
        self.left.off()
        # assure that both pins aren't on at the same time
        self.right.on()
        sleep(time)
        self.right.off()


class PWMCar:
    def __init__(self):
        self.forw = PWMOutputDevice(9)
        self.back = PWMOutputDevice(10)
        self.left = PWMOutputDevice(7)
        self.right = PWMOutputDevice(9)

    def forward(self, time=1, speed=1):
        self.back.off()
        # assure that both pins aren't on at the same time
        self.forw.value(speed)
        sleep(time)
        self.forw.off()

    def backward(self, time=1, speed=1):
        self.forw.off()
        # assure that both pins aren't on at the same time
        self.back.value(speed)
        sleep(time)
        self.back.off()

    def left(self, time=1, speed=1):
        self.right.off()
        # assure that both pins aren't on at the same time
        self.left.value(speed)
        sleep(time)
        self.left.off()

    def right(self, time=1, speed=1):
        self.left.off()
        # assure that both pins aren't on at the same time
        self.right.value(speed)
        sleep(time)
        self.right.off()


if __name__ == '__main__':
    car = PWMCar()
    car.forward(speed=0.5)
