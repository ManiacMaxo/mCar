use opencv::prelude::*;
use opencv::videoio;
use std::result::Result;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

pub struct Camera {
    capture_device: videoio::VideoCapture,
    buf: Vec<opencv::prelude::Mat>,
    channel: (Sender<()>, Receiver<()>),
}

impl Camera {
    pub fn new() -> Camera {
        let (tx, rx) = mpsc::channel();
        let capture_device = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();

        return Camera {
            capture_device,
            channel: (tx, rx),
            buf: Vec::with_capacity(60), // 30fps * 2 seconds,
        };
    }

    pub fn start(&mut self) -> Result<bool, opencv::Error> {
        match self.capture_device.open(0, videoio::CAP_ANY) {
            Ok(_) => {
                std::thread::spawn(|| self.poll());
                return Ok(true);
            }
            Err(e) => Err(e),
        }
    }

    fn poll(&mut self) {
        let (_, rx) = self.channel;
        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            let mut frame = Mat::default();
            if self.capture_device.read(&mut frame).is_err() {
                println!("opa");
                return;
            }

            // remove first element if buffer is full
            if self.buf.len() == self.buf.capacity() {
                self.buf.remove(0);
            }
            self.buf.push(frame);
        }
    }

    pub fn stop(&mut self) {
        self.channel.0.send(());
        self.capture_device.release().unwrap();
        self.buf.clear();
    }

    pub fn read(self) {
        // TODO: return next frame from buffer
    }
}
