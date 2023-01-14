use rust_gpiozero::Motor;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
pub struct Car {
    drive_motor: Arc<Mutex<Motor>>,
    turn_motor: Arc<Mutex<Motor>>,
    pub is_driving: AtomicBool,
}

impl Car {
    pub fn new() -> Car {
        Car {
            drive_motor: Arc::new(Mutex::new(Motor::new(9, 10))),
            turn_motor: Arc::new(Mutex::new(Motor::new(8, 7))),
            is_driving: AtomicBool::new(false),
        }
    }

    pub fn drive(&self, x: f32, y: f32) {
        let capped_x = x.max(-1.0).min(1.0);
        let capped_y = y.max(-1.0).min(1.0);

        let drive_motor = self.drive_motor.clone();
        let mut drive_motor = drive_motor.lock().unwrap();
        drive_motor.set_speed(capped_y.abs().into());

        if capped_y > 0.0 {
            drive_motor.forward();
        } else {
            drive_motor.backward();
        }

        let turn_motor = self.turn_motor.clone();
        let mut turn_motor = turn_motor.lock().unwrap();
        turn_motor.set_speed(capped_x.abs().into());

        if capped_x > 0.0 {
            turn_motor.forward();
        } else {
            turn_motor.backward();
        }
    }

    pub fn stop(&self) {
        let drive_motor = self.drive_motor.clone();
        let mut drive_motor = drive_motor.lock().unwrap();
        drive_motor.stop();

        let turn_motor = self.turn_motor.clone();
        let mut turn_motor = turn_motor.lock().unwrap();
        turn_motor.stop();
    }

    pub fn set_driving(&self, is_driving: bool) {
        self.is_driving.store(is_driving, Ordering::Relaxed);
    }
}
