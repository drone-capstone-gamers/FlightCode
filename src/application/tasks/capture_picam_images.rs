use std::process::Command;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::Duration;
use crate::application::data_manage::{DataSource, get_data_source_string, IncomingData};
use crate::application::DataCollector;
use crate::application::timer::TimedTask;
use opencv::{imgcodecs, core};
use opencv::prelude::{Mat};

pub struct CapturePiCamImages {
    storage_sender: SyncSender<IncomingData>
}

impl DataCollector for CapturePiCamImages {
    fn new(storage_sender: SyncSender<IncomingData>) -> Self {
        Self {
            storage_sender
        }
    }
}

impl TimedTask for CapturePiCamImages {
    fn execute(&mut self) -> () {
        let temp_proc_result = Command::new("python")
            .arg("helper-scripts/capture-pi-camera-raw-frame.py")
            .output();

        if temp_proc_result.is_err() {
            println!("Failed to capture picamera image!");
            thread::sleep(Duration::from_secs(10)); // TODO: only log every 10 secs but still reattempt as normal
            return;
        }

        let raw_image_bytes = temp_proc_result.unwrap().stdout;

        let frame = Mat::from_slice(raw_image_bytes.as_slice())
            .expect("Failed to create Mat from raw image bytes");

        let mut image_data = core::Vector::<u8>::new();

        imgcodecs::imencode(".png", &frame, &mut image_data, &core::Vector::<i32>::new()).expect("Failed to encode compressed thermal image into bytes");

        let ir_cam_incoming_data = IncomingData::new(DataSource::IrCamImage, None, Option::from(image_data.to_vec()));

        self.storage_sender.send(ir_cam_incoming_data)
            .expect(&*format!("Failed to send data into write queue: {}",
                              get_data_source_string(&DataSource::IrCamImage)));
    }
}