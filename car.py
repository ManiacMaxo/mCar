from gpiozero import Motor

from turn import Turn


class Car:
    def __init__(self):
        self.drive_motor = Motor(9, 10)
        self.turn_motor = Turn(8, 7)

    def forward(self, value=1):
        self.drive_motor.forward(value)

    def backward(self, value=1):
        self.drive_motor.backward(value)

    def left(self):
        self.turn_motor.left()

    def right(self):
        self.turn_motor.right()

    def stop(self):
        self.drive_motor.stop()
        self.turn_motor.stop()
