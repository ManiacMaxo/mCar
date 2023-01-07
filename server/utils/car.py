import warnings

try:
    from gpiozero import Device

    warnings.filterwarnings("ignore")
    Device._default_pin_factory()
except:
    from .motor import DummyMotor

    Motor = DummyMotor
else:
    from gpiozero import Motor

    warnings.resetwarnings()


class Car:
    def __init__(self):
        self.drive_motor = Motor(9, 10)
        self.turn_motor = Motor(8, 7)

    def forward(self, value=1):
        self.drive_motor.forward(value)

    def backward(self, value=1):
        self.drive_motor.backward(value)

    def left(self, value=1):
        self.turn_motor.forward(value)

    def right(self, value=1):
        self.turn_motor.backward(value)

    def drive(self, x, y):
        x = min(max(-1, x), 1)  # cap X between -1 and 1
        y = min(max(-1, y), 1)  # cap Y between -1 and 1
        if y > 0:
            self.forward(y)
        else:
            self.backward(-y)

        if x > 0:
            self.right(x)
        else:
            self.left(-x)

    def stop(self):
        self.drive_motor.stop()
        self.turn_motor.stop()


car = Car()
