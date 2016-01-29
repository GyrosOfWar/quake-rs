#[cfg(target_os="windows")] use self::win32::*;
#[cfg(any(target_os="unix", target_os="linux"))] use self::nix::*;
use std::time::Duration;
use util::DurationExt;

pub struct Timer {
    total_time_passed: i64,
    nanos_per_tick: f64
}

impl Timer {
    pub fn new() -> Timer {
        let res = get_timer_resolution() as f64;
        let nanos_per_tick = 1e9/res;
        Timer {
            total_time_passed: get_perf_counter(),
            nanos_per_tick: nanos_per_tick
        }
    }
    
    /// Returns the time since the last time get_time was called and 
    /// sets the internal time of the timer to the current time.
    pub fn get_time(&mut self) -> Duration {
        let now = get_perf_counter();
        let interval = now - self.total_time_passed;
        self.total_time_passed = now;
        
        let nanos: f64 = interval as f64 * self.nanos_per_tick;
        Duration::from_nanos(nanos as u64)
    }
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
    fn long_timer() {
        let mut timer = Timer::new();
        timer.get_time();
        thread::sleep(Duration::from_secs(2));
        let t = timer.get_time();
        let seconds = t.as_secs();
        assert!(seconds >= 2);
    }
    
    #[test]
    fn short_timer() {
        let mut timer = Timer::new();
        thread::sleep(Duration::from_millis(20));
        let duration = timer.get_time();
        
        let nanos = duration.subsec_nanos();
        let millis = nanos as f64 / 1e6;
        assert!(millis >= 20.0);
    }
}