use std::sync::{Arc, Mutex};
use std::thread;
use crate::application::data_manage::{DataSource, IncomingData};
use crate::application::tasks::pib_adapter::PibCommander;

pub fn spawn_payload_orientator(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>, pib_commander: Arc<PibCommander>) {
    thread::spawn(move || {
        payload_orientator_loop(current_data_storage, pib_commander);
    });
}

fn payload_orientator_loop(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>, pib_commander: Arc<PibCommander>) {
    loop {

    }
}
