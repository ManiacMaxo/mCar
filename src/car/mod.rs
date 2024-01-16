use std::panic::catch_unwind;

mod motor;
use motor::{DummyMotor, GenericMotor, Motor};

pub struct Car {
    drive_motor: Box<dyn GenericMotor>,
    turn_motor: Box<dyn GenericMotor>,
}

impl Car {
    pub fn new() -> Car {
        let drive_motor = catch_unwind(|| Motor::new(9, 10));
        let turn_motor = catch_unwind(|| Motor::new(8, 7));
        match drive_motor {
            Ok(_) => Car {
                drive_motor: Box::new(drive_motor.unwrap()),
                turn_motor: Box::new(turn_motor.unwrap()),
            },
            Err(_) => Car {
                drive_motor: Box::new(DummyMotor::new()),
                turn_motor: Box::new(DummyMotor::new()),
            },
        }
    }

    pub fn drive(&mut self, x: f64, y: f64) {
        let capped_x = x.max(-1.0).min(1.0);
        let capped_y = y.max(-1.0).min(1.0);

        if capped_y > 0.0 {
            self.drive_motor.forward(capped_y);
        } else {
            self.drive_motor.backward(-capped_y);
        }

        if capped_x > 0.0 {
            self.turn_motor.forward(capped_x);
        } else {
            self.turn_motor.backward(-capped_x);
        }
    }

    pub fn stop(&mut self) {
        self.drive_motor.stop();
        self.turn_motor.stop();
    }
}

unsafe impl Send for Car {}
