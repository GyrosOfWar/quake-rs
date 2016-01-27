#[allow(dead_code)]
#[cfg(target_os="windows")] pub use self::win32::Timer;

#[cfg(target_os="windows")]
mod win32 {
    use kernel32::*;
    
    pub struct Timer {
        tick: i64,
        tock: i64,
        seconds_per_tick: f64
    }
    
    impl Timer {
        pub fn new() -> Timer {
            let mut resolution = 0;
            let mut tick = 0;
            let mut tock = 0;
            unsafe {
                QueryPerformanceFrequency(&mut resolution);
                QueryPerformanceCounter(&mut tick);
                QueryPerformanceCounter(&mut tock);
            }
            
            Timer {
                seconds_per_tick: 1.0 / resolution as f64,
                tick: tick,
                tock: 0
            }
        }
        
        pub fn tick(&mut self) {
            unsafe { QueryPerformanceCounter(&mut self.tick); }
        }
        
        pub fn tock(&mut self) {
            unsafe { QueryPerformanceCounter(&mut self.tock); }
        }
        
        pub fn elapsed_seconds(&self) -> f64 {
            (self.tock - self.tick) as f64 * self.seconds_per_tick
        }
        
        pub fn elapsed_ticks(&self) -> i64 {
            self.tock - self.tick
        }
    }
}