///! stripped https://github.com/rahul-thakoor/rust_gpiozero
use rppal::gpio::{Gpio, IoPin, Mode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
struct DummyIoPin {}

#[derive(Debug)]
struct OutputDevice {
    pin: IoPin,
}

impl OutputDevice {
    pub fn new(pin: u8) -> OutputDevice {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => OutputDevice {
                    pin: pin.into_io(Mode::Output),
                },
            },
        }
    }
}

struct PWMOutputDevice {
    device: Arc<Mutex<OutputDevice>>,
    blinking: Arc<AtomicBool>,
    active_state: bool,
}

impl PWMOutputDevice {
    pub fn new(pin: u8) -> PWMOutputDevice {
        PWMOutputDevice {
            device: Arc::new(Mutex::new(OutputDevice::new(pin))),
            blinking: Arc::new(AtomicBool::new(false)),
            active_state: true,
        }
    }

    fn stop(&mut self) {
        self.blinking.clone().store(false, Ordering::SeqCst);
        if self.device.lock().unwrap().pin.clear_pwm().is_err() {
            println!("Could not clear pwm for pin");
        };
    }

    fn set_value(&mut self, value: f64) {
        if !(0.0..=1.0).contains(&value) {
            println!("Value must be between 0.0 and 1.0");
            return;
        }
        self.stop();
        if self.active_high() {
            self.device
                .lock()
                .unwrap()
                .pin
                .set_pwm_frequency(100.0, value)
                .unwrap()
        } else {
            self.device
                .lock()
                .unwrap()
                .pin
                .set_pwm_frequency(100.0, 1.0 - value)
                .unwrap()
        }
    }

    pub fn active_high(&self) -> bool {
        self.active_state
    }
}

struct MotorCompositeDevice(PWMOutputDevice, PWMOutputDevice);

pub struct Motor {
    devices: MotorCompositeDevice,
}

impl Motor {
    pub fn new(forward_pin: u8, backward_pin: u8) -> Motor {
        let forward = PWMOutputDevice::new(forward_pin);
        let backward = PWMOutputDevice::new(backward_pin);
        Motor {
            devices: MotorCompositeDevice(forward, backward),
        }
    }

    pub fn forward(&mut self, speed: f64) {
        if !(speed >= 0.0 && speed <= 1.0) {
            println!("Speed must be between 0.0 and 1.0");
            return;
        }

        self.devices.1.set_value(0.0);
        self.devices.0.set_value(speed);
    }

    pub fn backward(&mut self, speed: f64) {
        if !(speed >= 0.0 && speed <= 1.0) {
            println!("Speed must be between 0.0 and 1.0");
            return;
        }

        self.devices.0.set_value(0.0);
        self.devices.1.set_value(speed);
    }

    pub fn stop(&mut self) {
        self.devices.0.set_value(0.0);
        self.devices.1.set_value(0.0);
    }
}
