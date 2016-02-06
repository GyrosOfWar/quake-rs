#[cfg(target_os="windows")] use self::win32::*;
#[cfg(any(target_os="unix", target_os="linux"))] use self::nix::*;

pub struct Timer {
    last_tick: i64,
    seconds_per_tick: f64,
    total_time: f64,
    max_frame_time: f64,
    last_interval: f64
}

impl Timer {
    pub fn new(max_framerate: i32) -> Timer {
        let res = get_timer_resolution() as f64;
        let spt = 1.0 / res;
        let max_frame_time = 1.0 / max_framerate as f64;
        Timer {
            last_tick: get_perf_counter(),
            seconds_per_tick: spt,
            total_time: 0.0,
            max_frame_time: max_frame_time,
            last_interval: 0.0
        }
    }
    
    pub fn filter_time(&mut self) -> bool {
        let now = get_perf_counter();
        let interval_seconds = (now - self.last_tick) as f64 * self.seconds_per_tick;
        self.total_time += interval_seconds;
        if interval_seconds > self.max_frame_time {
            self.last_tick = now;
            self.last_interval = interval_seconds;
            true
        } else {
            false
        }
    }
    
    pub fn total_seconds(&self) -> f64 { self.total_time }
    
    pub fn last_interval(&self) -> f64 { self.last_interval }
}

#[cfg(target_os="windows")]
mod win32 {
    use kernel32::*;
    
    pub fn get_perf_counter() -> i64 {
        let mut t = 0;
        unsafe { QueryPerformanceCounter(&mut t); }
        t
    }
    
    pub fn get_timer_resolution() -> i64 {
        let mut t = 0;
        unsafe { QueryPerformanceFrequency(&mut t); }
        t
    }
}

#[cfg(any(target_os="unix", target_os="linux"))]
mod nix {
    pub fn get_perf_counter() -> i64 {
        unimplemented!()
    }
    
    pub fn get_timer_resolution() -> i64 {
        unimplemented!()
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
        assert_eq!(timer.filter_time(), false);
        thread::sleep(Duration::from_millis(30));
        assert_eq!(timer.filter_time(), true);
    }

}