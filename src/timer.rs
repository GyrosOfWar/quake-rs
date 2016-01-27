#[allow(dead_code)]
#[cfg(target_os="windows")] pub use self::win32::Timer;

#[cfg(target_os="windows")]
mod win32 {
    use kernel32::*;
    
    fn get_perf_counter() -> i64 {
        let mut t = 0;
        unsafe { QueryPerformanceCounter(&mut t); }
        t
    }
        
    pub struct Timer {
        tick: i64,
        tock: i64,
        seconds_per_tick: f64
    }
    
    impl Timer {
        pub fn new() -> Timer {
            let mut resolution = 0;
            unsafe {
                QueryPerformanceFrequency(&mut resolution);
            }
            
            Timer {
                seconds_per_tick: 1.0 / resolution as f64,
                tick: get_perf_counter(),
                tock: 0
            }
        }
        
        pub fn tick(&mut self) {
            self.tick = get_perf_counter()
        }
        
        pub fn tock(&mut self) {
            self.tock = get_perf_counter()
        }
        
        pub fn elapsed_seconds(&self) -> f64 {
            (self.tock - self.tick) as f64 * self.seconds_per_tick
        }
        
        pub fn elapsed_ticks(&self) -> i64 {
            self.tock - self.tick
        }
    }
}