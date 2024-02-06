use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

// Runs at every time interval
pub trait TimedTask {
    fn execute(&self) -> ();
}

pub struct Timer {
    identification: String,
    interval: Duration,
    last_update: Instant
}

impl Timer {
    pub fn new(identification: String, interval: Duration) -> Self {
        Self {
            identification,
            last_update: Instant::now(),
            interval
        }
    }
}

pub fn spawn_timer(timer: Timer, task: Box<dyn TimedTask + Send>) ->  Sender<bool> {
    let (kill_sender, kill_recv) = mpsc::channel();

    thread::spawn(|| {
        timer_loop(timer, task, kill_recv);
    });

    return kill_sender;
}

fn timer_loop(mut timer: Timer, task: Box<dyn TimedTask>, kill_recv: Receiver<bool>) -> () {
    loop {
        let current_time = Instant::now();
        let duration = current_time.duration_since(timer.last_update);
        if duration.ge(&timer.interval) {
            task.execute();
            timer.last_update = current_time;
        }

        // Halt task upon receiving kill signal
        let kill_signal = kill_recv.recv_timeout(Duration::from_millis(25));
        if kill_signal.is_ok() && kill_signal.unwrap() == true {
            println!("Task {} was killed by signal", timer.identification);
            break;
        }
    }
}
