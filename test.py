from time import sleep

from gpiozero import DigitalOutputDevice, PWMOutputDevice


class Car:
    def __init__(self, pwm=False):
        self._pinClass = PWMOutputDevice if pwm else DigitalOutputDevice
        self.forw = self._pinClass(9)
        self.back = self._pinClass(10)
        self.left = self._pinClass(7)
        self.right = self._pinClass(8)

    def forward(self, time=1, speed=1):
        if (self._pwm):
            assert speed == 1 or speed == 0

        self.back.off()
        # assure that both pins aren't on at the same time
        self.back.value(speed) if self._pwm else self.back.on()
        sleep(time)
        self.forw.off()

    def backward(self, time=1, speed=1):
        if (self._pwm):
            assert speed == 1 or speed == 0
        self.forw.off()
        # assure that both pins aren't on at the same time
        self.back.value(speed) if self._pwm else self.back.on()
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
    car.forward()
