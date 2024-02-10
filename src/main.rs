mod application;

extern crate ll_protocol;

use std::{thread, time};

fn main() {
    application::start_collection_tasks();

    loop {
        thread::sleep(time::Duration::from_millis(1000));
    }
}
