from time import sleep

from gpiozero import DigitalOutputDevice


class Car:
    def __init__(self):
        self.forw = DigitalOutputDevice(7)
        self.back = DigitalOutputDevice(8)
        self.left = DigitalOutputDevice(9)
        self.right = DigitalOutputDevice(10)

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


if __name__ == '__main__':
    car = Car()
    car.forward(10000)
