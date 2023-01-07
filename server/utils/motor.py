class DummyMotor:
    is_active = False

    def __init__(self, *args, **kwargs):
        pass

    def forward(self, *args, **kwargs):
        pass

    def backward(self, *args, **kwargs):
        pass

    def reverse(self):
        pass

    def stop(self):
        pass

    @property
    def value(self):
        """
        Represents the speed of the motor as a floating point value between -1
        (full speed backward) and 1 (full speed forward), with 0 representing
        stopped.
        """
        return None

    @value.setter
    def value(self, value):
        pass
