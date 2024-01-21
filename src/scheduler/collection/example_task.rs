use std::sync::mpsc::Sender;
use json::object;
use crate::scheduler::collection::data_manage::{DataSource, get_data_source_string, IncomingData};
use crate::scheduler::timer::TimedTask;

pub struct ExampleTask {
    storage_sender: Sender<IncomingData>
}

impl TimedTask for ExampleTask {
    fn new(storage_sender: Sender<IncomingData>) -> Self {
        Self {
            storage_sender
        }
    }

    fn execute(&self) -> () {
        let example_payload = object!{
            test: "This is a test!"
        };

        let example_data = IncomingData::new(DataSource::Example, Option::from(example_payload), None);

        self.storage_sender.send(example_data)
            .expect(&*format!("Failed to send data into write queue: {}",
                              get_data_source_string(DataSource::Example)));
    }
}