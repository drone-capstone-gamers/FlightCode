use std::time::Duration;
use crate::scheduler::collection_tasks::example_task::ExampleTask;
use crate::scheduler::timer::{spawn_timer, Timer};

mod timer;
mod collection_tasks;

pub fn start_collection_tasks() {
    let example_task = ExampleTask {};
    let example_timer = Timer::new("Example_Task".to_string(), Duration::from_secs(1));
    spawn_timer(example_timer, example_task);
}
