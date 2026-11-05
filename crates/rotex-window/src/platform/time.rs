//! Monotonic clock and a one-second frame-rate meter.

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::{Duration, Instant};
#[cfg(target_arch = "wasm32")]
pub use web_time::{Duration, Instant};

/// Counts rendered frames and reports a rate about once per second.
#[derive(Debug)]
pub struct FpsMeter {
    window_start: Option<Instant>,
    frames: u32,
}

impl FpsMeter {
    /// Starts with no samples.
    pub const fn new() -> Self {
        Self {
            window_start: None,
            frames: 0,
        }
    }

    /// Records one rendered frame.
    ///
    /// Returns the frames per second for the window that just closed, about once
    /// per second. Other calls return `None`.
    pub fn tick(&mut self) -> Option<f32> {
        self.frames = self.frames.saturating_add(1);
        let now = Instant::now();
        let start = *self.window_start.get_or_insert(now);
        let elapsed = now.duration_since(start).as_secs_f32();
        if elapsed < 1.0 {
            return None;
        }
        let fps = self.frames as f32 / elapsed;
        self.frames = 0;
        self.window_start = Some(now);
        Some(fps)
    }
}

impl Default for FpsMeter {
    fn default() -> Self {
        Self::new()
    }
}
