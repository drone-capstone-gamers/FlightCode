use crate::scheduler::timer::TimedTask;

pub struct ExampleTask {}

impl TimedTask for ExampleTask {
    fn execute(&self) -> () {
        println!("This is an example task that has just been executed!");
    }
}