use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::thread::sleep;
use std::time::Duration;
use crate::application::tasks::capture_go_pro_images::GoProTask;
use crate::application::data_manage::{IncomingData, spawn_data_manager};
use crate::application::tasks::example_task::ExampleTask;
use crate::application::timer::{spawn_timer, Timer};

mod timer;
mod tasks;
mod data_manage;

pub trait DataCollector {
    fn new(storage_sender: SyncSender<IncomingData>) -> Self;
}

pub fn start_collection_tasks() {
    let (queue_sender, queue_recv) = mpsc::sync_channel(2);

    spawn_data_manager(queue_recv);

    let example_task = ExampleTask::new(queue_sender.clone());
    let example_timer = Timer::new("Example_Task".to_string(), Duration::from_secs(1));
    let example_timer_handler = spawn_timer(example_timer, Box::from(example_task));

    let example_task1 = ExampleTask::new(queue_sender.clone());
    let example_timer1 = Timer::new("Example_Task1".to_string(), Duration::from_secs(2));
    let example_timer1_handler = spawn_timer(example_timer1, Box::from(example_task1));

    let gopro_task = GoProTask::new();
    let gopro_timer = Timer::new("GoProControl".to_string(), Duration::from_secs(5));
    let gopro_handler = spawn_timer(gopro_timer, Box::from(gopro_task));

    sleep(Duration::from_secs(10));
    example_timer_handler.send(true).expect("Failed to send kill signal to collection task");
    example_timer1_handler.send(true).expect("Failed to send kill signal to collection task");
}
