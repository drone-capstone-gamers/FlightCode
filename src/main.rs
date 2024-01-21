mod scheduler;

extern crate ll_protocol;

fn main() {
    scheduler::start_collection_tasks();

    loop {}
}
