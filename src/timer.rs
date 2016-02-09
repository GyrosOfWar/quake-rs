use std::time::{Duration, Instant};
use util::DurationExt;

#[derive(Debug)]
pub struct Timer {
    last_tick: Instant,
    total_time: Duration,
    max_frame_time: f64,
    last_interval: Duration
}

impl Timer {
    pub fn new(max_framerate: i32) -> Timer {
        let max_frame_time = 1.0 / max_framerate as f64;
        Timer {
            last_tick: Instant::now(),
            total_time: Duration::from_millis(0),
            max_frame_time: max_frame_time,
            last_interval: Duration::from_millis(0)
        }
    }
    
    pub fn filter_time(&mut self) -> Option<f64> {
        let interval = self.last_tick.elapsed();
        let interval_seconds = self.last_tick.elapsed().seconds();
        let now = Instant::now();
        self.total_time = self.total_time + interval;
        if interval_seconds > self.max_frame_time {
            self.last_tick = now;
            self.last_interval = interval;
            Some(interval_seconds)
        } else {
            None
        }
    }
    
    pub fn total_seconds(&self) -> f64 { self.total_time.seconds() }
    
    pub fn last_interval(&self) -> f64 { self.last_interval.seconds() }
}

#[cfg(test)]
mod tests {
    use super::Timer;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_tick() {
        let mut timer = Timer::new(2);
        thread::sleep(Duration::from_millis(480));
        assert_eq!(timer.filter_time().is_some(), false);
        thread::sleep(Duration::from_millis(30));
        assert_eq!(timer.filter_time().is_some(), true);
    }

}