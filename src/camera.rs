use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera as NokhwaCamera,
};
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tokio::sync::broadcast;

pub struct Camera {
    index: CameraIndex,
    subscribers: Arc<Mutex<u32>>,
    tx: broadcast::Sender<Vec<u8>>,
}

impl Camera {
    pub fn new(cam_index: u32) -> Self {
        let index = CameraIndex::Index(cam_index);

        let (tx, _) = broadcast::channel::<Vec<u8>>(5);

        Self {
            index,
            subscribers: Arc::new(Mutex::new(0)),
            tx,
        }
    }

    pub fn start(&self) {
        let requested =
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let index = self.index.clone();

        let subscribers = self.subscribers.clone();
        let tx = self.tx.clone();

        thread::spawn(move || {
            let mut camera = NokhwaCamera::new(index, requested).unwrap();
            camera.open_stream().unwrap();

            loop {
                if *subscribers.lock().unwrap() == 0 {
                    let frame = camera.frame().unwrap();
                    tx.send(frame.buffer().to_vec()).unwrap();
                }
            }
        });
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Vec<u8>> {
        {
            let mut subscribers = self.subscribers.lock().unwrap();
            *subscribers += 1;
        }

        self.tx.subscribe()
    }

    pub fn unsubscribe(self) {
        {
            let mut subscribers = self.subscribers.lock().unwrap();
            *subscribers -= 1;
        }
    }
}
