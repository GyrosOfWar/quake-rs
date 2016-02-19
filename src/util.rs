use std::time::Duration;
use std::ops;

/// Provides some additional conversions for Duration types.
pub trait DurationExt {
    /// Returns the whole duration in seconds, including the nano-second
    /// precision.
    fn seconds(&self) -> f64;

    /// Returns the whole duration in milliseconds, including
    /// the nano-second precision.
    fn millis(&self) -> f64;

    /// Creates a time from nanoseconds. (since the Duration::new function only
    // takes nanoseconds as a u32, which can easily overflow)
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

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    _unused: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            _unused: 0
        }
    }
}

const EPSILON: f32 = 0.0001;

pub fn step(start: f32, end: f32) -> F32Step {
    assert!(end >= start);

    F32Step {
        start: start,
        end: end,
        current: start
    }
}

fn almost_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub struct F32Step {
    start: f32,
    end: f32,
    current: f32
}

impl Iterator for F32Step {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if almost_eq(self.current, self.end) {
            None
        } else {
            let c = self.current;
            self.current += 1.0;
            Some(c)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let d = (self.end - self.start) as usize;

        (d, Some(d))
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use super::DurationExt;

    #[test]
    fn test_from_nanos() {
        // 120 seconds
        let nanos: u64 = 1_000_000_000 * 120;
        let duration = Duration::from_nanos(nanos);
        assert_eq!(duration.as_secs(), 120);
        assert_eq!(duration.subsec_nanos(), 0);
    }

    #[test]
    fn test_from_nanos_2() {
        let nanos: u64 = 3_000_000_000 + 64;
        let duration = Duration::from_nanos(nanos);

        assert_eq!(duration.as_secs(), 3);
        assert_eq!(duration.subsec_nanos(), 64);
    }

    #[test]
    fn test_to_seconds() {
        let duration = Duration::new(3, 500_000_000);
        let secs = duration.seconds();
        assert_eq!(secs, 3.5);
    }

    #[test]
    fn test_to_millis() {
        let duration = Duration::new(0, 500_000_000);
        let millis = duration.millis();
        assert_eq!(millis, 500.0);
    }
}
