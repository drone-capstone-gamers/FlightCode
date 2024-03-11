use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crate::application::data_manage::{DataSource, IncomingData};
use crate::application::tasks::pib_adapter::PibCommander;

pub fn spawn_payload_orientator(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>, pib_commander: Arc<PibCommander>) {
    thread::spawn(move || {
        payload_orientator_loop(current_data_storage, pib_commander);
    });
}

fn payload_orientator_loop(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>, pib_commander: Arc<PibCommander>) {
    let mut test_value: i8 = -128;

    loop {
        test_value = if test_value == -128 {127} else {-128};

        pib_commander.put_servo_set(test_value);

        sleep(Duration::from_secs(1));
    }
}
