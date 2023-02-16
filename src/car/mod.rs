use std::{
    panic::catch_unwind,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

mod motor;
use motor::{DummyMotor, GenericMotor, Motor};

pub struct Car {
    drive_motor: Arc<Mutex<dyn GenericMotor>>,
    turn_motor: Arc<Mutex<dyn GenericMotor>>,
    pub is_driving: AtomicBool,
}

impl Car {
    pub fn new() -> Car {
        let drive_motor = catch_unwind(|| Motor::new(9, 10));
        let turn_motor = catch_unwind(|| Motor::new(8, 7));
        match drive_motor {
            Ok(_) => Car {
                drive_motor: Arc::new(Mutex::new(drive_motor.unwrap())),
                turn_motor: Arc::new(Mutex::new(turn_motor.unwrap())),
                is_driving: AtomicBool::new(false),
            },
            Err(_) => Car {
                drive_motor: Arc::new(Mutex::new(DummyMotor::new())),
                turn_motor: Arc::new(Mutex::new(DummyMotor::new())),
                is_driving: AtomicBool::new(false),
            },
        }
    }

    pub fn drive(&self, x: f64, y: f64) {
        let capped_x = x.max(-1.0).min(1.0);
        let capped_y = y.max(-1.0).min(1.0);

        let drive_motor = self.drive_motor.clone();
        let mut drive_motor = drive_motor.lock().unwrap();

        if capped_y > 0.0 {
            drive_motor.forward(capped_y);
        } else {
            drive_motor.backward(-capped_y);
        }

        let turn_motor = self.turn_motor.clone();
        let mut turn_motor = turn_motor.lock().unwrap();

        if capped_x > 0.0 {
            turn_motor.forward(capped_x);
        } else {
            turn_motor.backward(-capped_x);
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
