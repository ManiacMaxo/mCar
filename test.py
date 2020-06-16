from time import sleep

from car import Car

car = Car()

car.forward(0.5)
car.left()
sleep(2)
car.stop()
print('stopped')
sleep(1)
car.backward(1)
car.right()
sleep(2)
car.stop()
