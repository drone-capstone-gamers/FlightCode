use std::sync::mpsc::SyncSender;
use opencv::core::MatTraitConst;
use crate::application::data_manage::{DataSource, get_data_source_string, IncomingData};
use crate::application::DataCollector;
use crate::application::timer::TimedTask;
use opencv::{imgcodecs, videoio, core};
use opencv::prelude::Mat;
use opencv::videoio::VideoCaptureTrait;

pub struct CaptureIrImages {
    storage_sender: SyncSender<IncomingData>,
    capture: Option<videoio::VideoCapture>
}

impl DataCollector for CaptureIrImages {
    fn new(storage_sender: SyncSender<IncomingData>) -> Self {
        Self {
            storage_sender,
            capture: None
        }
    }
}

impl TimedTask for CaptureIrImages {
    fn execute(&mut self) -> () {
        if self.capture.is_none() {
            // TODO: find a way to differentiate cameras
            let mut new_capture = videoio::VideoCapture::new(0,videoio::CAP_ANY);

            if new_capture.is_ok() {
                self.capture = Option::from(new_capture.unwrap());
            } else {
                println!("Thermal camera not connected!");
                return;
            }
        }

        let mut frame = Mat::default();
        self.capture.as_mut().unwrap().read(&mut frame).unwrap();

        if frame.empty() {
            println!("End of thermal camera stream!");
            return;
        }

        let mut image_data = core::Vector::<u8>::new();

        imgcodecs::imencode(".png", &frame, &mut image_data, &core::Vector::<i32>::new()).expect("Failed to encode compressed thermal image into bytes");

        let ir_cam_incoming_data = IncomingData::new(DataSource::IrCamImage, None, Option::from(image_data.to_vec()));

        self.storage_sender.send(ir_cam_incoming_data)
            .expect(&*format!("Failed to send data into write queue: {}",
                              get_data_source_string(&DataSource::IrCamImage)));
    }
}