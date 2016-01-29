use std::time::Duration;

/// Provides some additional conversions for Duration types. 
pub trait DurationExt {
    /// Returns the whole duration in seconds, including the nano-second 
    /// precision.
    fn seconds(&self) -> f64;
    
    /// Returns the whole duration in milliseconds, including 
    /// the nano-second precision. 
    fn millis(&self) -> f64;
    
    /// Creates a time from nanoseconds
    fn from_nanos(nanos: u64) -> Duration;
}

impl DurationExt for Duration {
    #[inline]
    fn seconds(&self) -> f64 {
        self.as_secs() as f64 + self.subsec_nanos() as f64 / 1e9
    }
    
    #[inline]
    fn millis(&self) -> f64 {
        self.as_secs() as f64 * 1000.0 + (self.subsec_nanos() as f64 / 1e6)
    }
    
    #[inline]
    fn from_nanos(nanos: u64) -> Duration {
        if nanos > 1_000_000_000 {
            let seconds = nanos / 1_000_000_000;
            let nanos = nanos as u64 - (seconds as u64 * 1_000_000_000);
            Duration::new(seconds, nanos as u32)
        } else {
            Duration::new(0, nanos as u32)
        }
        
    }
}