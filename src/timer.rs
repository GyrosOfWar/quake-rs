use std::time::{Duration, Instant};
use util::DurationExt;

#[derive(Debug)]
pub struct Timer {
    last_frame: Instant,
    total_time: Duration,
    max_frame_time: f64
}

impl Timer {
    pub fn new(max_framerate: i32) -> Timer {
        let max_frame_time = 1.0 / max_framerate as f64;
        let now = Instant::now();
        Timer {
            last_frame: now,
            total_time: Duration::from_millis(0),
            max_frame_time: max_frame_time
        }
    }
    
    pub fn total_time(&self) -> Duration { self.total_time }
    
    pub fn filter_time(&mut self) -> Option<f64> {
        let now = Instant::now();
        
        // Elapsed time since the last time a frame was rendered/simulated
        let d_last_frame = self.last_frame.elapsed();
        let d_secs = d_last_frame.seconds();
        
        if d_secs > self.max_frame_time {
            self.total_time = self.total_time + d_last_frame;
            self.last_frame = now;
            Some(d_secs)
        } else {        
            None
        }
    }
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