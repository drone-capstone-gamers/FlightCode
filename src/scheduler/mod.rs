use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;
use crate::scheduler::collection::data_manage::spawn_data_manager;
use crate::scheduler::collection::example_task::ExampleTask;
use crate::scheduler::timer::{spawn_timer, TimedTask, Timer};

mod timer;
mod collection;

pub fn start_collection_tasks() {
    let (queue_sender, queue_recv) = mpsc::sync_channel(2);

    spawn_data_manager(queue_recv);

    let example_task = ExampleTask::new(queue_sender.clone());
    let example_timer = Timer::new("Example_Task".to_string(), Duration::from_secs(1));
    let example_timer_handler = spawn_timer(example_timer, example_task);

    let example_task1 = ExampleTask::new(queue_sender.clone());
    let example_timer1 = Timer::new("Example_Task1".to_string(), Duration::from_secs(2));
    let example_timer1_handler = spawn_timer(example_timer1, example_task1);

    sleep(Duration::from_secs(10));
    example_timer_handler.send(true).expect("Failed to send kill signal to collection task");
    example_timer1_handler.send(true).expect("Failed to send kill signal to collection task");
}
