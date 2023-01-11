use rust_gpiozero::Motor;

pub struct Car {
    drive_motor: Motor,
    turn_motor: Motor,
}

impl Car {
    pub fn new() -> Car {
        Car {
            drive_motor: Motor(9, 10),
            turn_motor: Motor(8, 7),
        }
    }

    pub fn drive(&self, x: f32, y: f32) {
        let capped_x = x.max(-1.0).min(1.0);
        let capped_y = y.max(-1.0).min(1.0);

        self.drive_motor.set_speed(capped_y.abs());

        if capped_y > 0.0 {
            self.drive_motor.forward();
        } else {
            self.drive_motor.backward();
        }

        self.turn_motor.set_speed(capped_x.abs());
        if capped_x > 0.0 {
            self.turn_motor.forward();
        } else {
            self.turn_motor.backward();
        }
    }

    pub fn stop(&self) {
        self.drive_motor.stop();
        self.turn_motor.stop();
    }
}
