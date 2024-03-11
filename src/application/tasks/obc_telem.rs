use std::process::Command;
use std::sync::mpsc::SyncSender;
use json::object;
use crate::application::data_manage::{DataSource, get_data_source_string, IncomingData};
use crate::application::DataCollector;
use crate::application::timer::TimedTask;

pub struct ObcTelem {
    storage_sender: SyncSender<IncomingData>
}

impl ObcTelem {
    fn get_temperature() -> String {
        let temp_proc_result = Command::new("vcgencmd")
            .arg("measure_temp")
            .output();

        if temp_proc_result.is_err() {
            println!("Failed to call cmd to measure OBC temp!");
            return "".to_string();
        }
        let temp_proc = temp_proc_result.unwrap();

        let temp_string: String = temp_proc.stdout.iter().map(|&c| c as char).collect();

        let temp_string_split: Vec<&str> = temp_string.split("=").collect();
        if temp_string_split.len() != 2 {
            println!("Failed to parse command response for OBC temp!");
            return "".to_string();
        }

        return temp_string_split[1].strip_suffix("'C\n").unwrap().to_string();
    }

    fn get_storage_left() -> String {
        let storage_proc_result = Command::new("df")
            .arg("-h")
            .arg("/")
            .output();

        if storage_proc_result.is_err() {
            println!("Failed to call cmd to check storage space!");
            return "".to_string();
        }
        let storage_proc = storage_proc_result.unwrap();

        let storage_output: String = storage_proc.stdout.iter().map(|&c| c as char).collect();

        let storage_split: Vec<&str> = storage_output.split("\n").collect();

        if storage_split.len() < 2 {
            println!("Failed to parse command response for OBC storage!");
            return "".to_string();
        }

        let storage_split: Vec<&str> = storage_split[1].split_whitespace().collect();

        if storage_split.len() < 6 {
            println!("Failed to parse command response for OBC storage!");
            return "".to_string();
        }

        return format!("{}/{}", storage_split[2], storage_split[1]);
    }
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
        let temp_value = ObcTelem::get_temperature();

        let storage_space_remaining = ObcTelem::get_storage_left();

        let telem_json = object!{
            core_temperature: temp_value,
            storage_space: storage_space_remaining
        };

        let obc_telem = IncomingData::new(DataSource::ObcTelemetry, Option::from(telem_json), None);

        self.storage_sender.send(obc_telem)
            .expect(&*format!("Failed to send data into write queue: {}",
                              get_data_source_string(&DataSource::ObcTelemetry)));
    }
}
