use std::time::{Duration, SystemTime};

type TaskRoutine = fn() -> bool;

// Runs at every time interval
struct TaskConfig {
    interval: Duration,
    routine: TaskRoutine
}

enum TaskPriority {
    Level0,
    Level1,
    Level2,
    Level3
}

struct Task {
    priority: TaskPriority,
    config: TaskConfig,
    last_update: SystemTime,
}

impl Task {
    fn new(&self, routine: TaskRoutine) -> Self {
        Self {
            priority: TaskPriority::Level3,
            config: TaskConfig {
                interval: Duration::MAX,
                routine,
            },
            last_update: SystemTime::now(),
        }
    }

    fn update(&mut self) -> bool {
        let mut updated = false;

        let current_time = SystemTime::now();
        if current_time.duration_since(self.last_update).expect("REASON").ge(&self.config.interval) {
            updated = (self.config.routine)();
            self.last_update = current_time;
        }

        return updated;
    }
}
