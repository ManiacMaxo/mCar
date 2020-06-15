#!/usr/bin/env python3
# CamJam EduKit 3 - Robotics
# Worksheet 7 - Controlling the motors with PWM

import time  # Import the Time library
from gpiozero import CamJamKitRobot  # Import the GPIO Zero Library CamJam library

robot = CamJamKitRobot()


for i in range (10):

    # Set the relative speeds of the two motors, between 0.0 and 1.0
    motorspeed = 0.1 * (i+1)
    motorforward = (motorspeed, 0)
    motorbackward = (-motorspeed, 0)

    print("Forward", motorspeed);
    robot.value = motorforward
    time.sleep(1)
    
    print("Pause", motorspeed);
    robot.stop()
    time.sleep(1)

    print("Backward", motorspeed);    
    robot.value = motorbackward
    time.sleep(1)  # Pause for 1 second

    print("Pause", motorspeed);    
    robot.stop()
    time.sleep(1)
