from gpiozero import Motor

__all__ = [Turn]


class Turn(Motor):
    def __init__(self, left=8, right=7):
        super().__init__(left, right)

    def left(self):
        super().forward()

    def right(self):
        super().backward()
