mod scheduler;

extern crate ll_protocol;

use std::{thread, time};

fn main() {
  loop {
    println!("THIS IS A TEST FLIGHTCODE LOOP");
    thread::sleep(time::Duration::from_millis(1000));
  }
}
