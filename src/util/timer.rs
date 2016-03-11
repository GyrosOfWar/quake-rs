use std::time::{Duration, Instant};

const MAX_FRAMERATE: f32 = 72.0;

pub struct Timer {
    start: Instant,
    last_frame: Instant,
    total: Duration,
    frame_duration: Duration,
    unlocked: bool,
}

impl Timer {
    pub fn new(unlocked: bool) -> Timer {
        Timer {
            start: Instant::now(),
            last_frame: Instant::now(),
            total: Duration::from_millis(0),
            frame_duration: Duration::new(0, (1e9 / MAX_FRAMERATE) as u32),
            unlocked: unlocked,
        }
    }

    pub fn step(&mut self) -> Option<Duration> {
        let now = Instant::now();
        let timestep = now.duration_since(self.last_frame);
        self.total = self.start.elapsed();
        if self.unlocked || timestep > self.frame_duration {
            self.last_frame = now;
            Some(timestep)
        } else {
            None
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_tick() {
        let mut timer = Timer::new(false);
        thread::sleep(Duration::from_millis(5));
        assert_eq!(timer.step().is_some(), false);
        thread::sleep(Duration::from_millis(9));
        assert_eq!(timer.step().is_some(), true);
    }
}
