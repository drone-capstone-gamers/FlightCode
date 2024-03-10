use std::process::{Command, Stdio};
use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::Duration;
use std::io::Read;
use crate::application::data_manage::{DataSource, get_data_source_string, IncomingData};
use crate::application::DataCollector;
use crate::application::timer::TimedTask;

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
        let cam_proc_result = Command::new("python")
            .arg("helper-scripts/capture-pi-camera-raw-frame.py")
            .output();

        if cam_proc_result.is_err() {
            println!("Failed to capture picamera image!");
            thread::sleep(Duration::from_secs(10)); // TODO: only log every 10 secs but still reattempt as normal
            return;
        }

        let cam_proc = cam_proc_result.unwrap();

        let image_data = cam_proc.stdout;

        let pi_cam_incoming_data = IncomingData::new(DataSource::PiCamImage, None, Option::from(image_data));

        self.storage_sender.send(pi_cam_incoming_data)
            .expect(&*format!("Failed to send data into write queue: {}",
                              get_data_source_string(&DataSource::PiCamImage)));
    }
}
