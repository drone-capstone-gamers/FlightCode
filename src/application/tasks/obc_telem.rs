use std::process::Command;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::Duration;
use json::object;
use crate::application::data_manage::{DataSource, get_data_source_string, IncomingData};
use crate::application::DataCollector;
use crate::application::timer::TimedTask;

pub struct ObcTelem {
    storage_sender: SyncSender<IncomingData>
}

impl DataCollector for ObcTelem {
    fn new(storage_sender: SyncSender<IncomingData>) -> Self {
        Self {
            storage_sender
        }
    }
}

impl TimedTask for ObcTelem {
    fn execute(&mut self) -> () {
        let temp_proc_result = Command::new("/opt/vc/bin/vcgencmd")
            .arg("measure_temp")
            .output();

        if temp_proc_result.is_err() {
            println!("Failed to call cmd to measure OBC temp!");
            thread::sleep(Duration::from_secs(10)); // TODO: only log every 10 secs but still reattempt as normal
            return;
        }
        let temp_proc = temp_proc_result.unwrap();

        let temp_string: String = temp_proc.stdout.iter().map(|&c| c as char).collect();

        let temp_string_split: Vec<&str> = temp_string.split("=").collect();
        if temp_string.len() != 2 {
            println!("Failed to parse command response for OBC temp!");
            return;
        }

        let temp_value = temp_string_split[1];

        let telem_json = object!{
            core_temperature: temp_value
        };

        let obc_telem = IncomingData::new(DataSource::ObcTelemetry, Option::from(telem_json), None);

        self.storage_sender.send(obc_telem)
            .expect(&*format!("Failed to send data into write queue: {}",
                              get_data_source_string(&DataSource::ObcTelemetry)));
    }
}