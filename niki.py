#!/usr/bin/env python3
# CamJam EduKit 3 - Robotics
# Worksheet 7 - Controlling the motors with PWM

import time  # Import the Time library
import numpy as np
from gpiozero import CamJamKitRobot  # Import the GPIO Zero Library CamJam library

robot = CamJamKitRobot()


for motorspeed in np.arange(0.0, 1.0, 0.1):

    # Set the relative speeds of the two motors, between 0.0 and 1.0
    motorforward = (motorspeed, 0)
    motorbackward = (-motorspeed, 0)

    print("Motor Speed:", motorspeed);

    print("Forward");
    robot.value = motorforward
    time.sleep(1)
    
    print("Pause");
    robot.stop()
    time.sleep(1)

    print("Backward");
    robot.value = motorbackward
    time.sleep(1)  # Pause for 1 second

    print("Pause\n");
    robot.stop()
    time.sleep(1)
