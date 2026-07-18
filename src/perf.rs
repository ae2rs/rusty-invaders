use std::time::{Duration, Instant};

pub struct FrameTimer {
    fps_window_start: Instant,
    frame_count: u32,
    displayed_fps: f64,
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            fps_window_start: Instant::now(),
            frame_count: 0,
            displayed_fps: 0.0,
        }
    }

    pub fn end_frame(&mut self) -> Option<f64> {
        self.frame_count += 1;

        let elapsed = self.fps_window_start.elapsed();

        if elapsed >= Duration::from_secs(1) {
            self.displayed_fps = self.frame_count as f64 / elapsed.as_secs_f64();

            self.frame_count = 0;
            self.fps_window_start = Instant::now();

            Some(self.displayed_fps)
        } else {
            None
        }
    }
}

pub struct TickTimer {
    tick_duration: Duration,
    last_update: Instant,
    accumulator: Duration,
}

impl TickTimer {
    pub fn new(tick_duration: Duration) -> Self {
        Self {
            tick_duration,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
        }
    }

    /// Returns true once per fully elapsed tick duration.
    pub fn consume_tick(&mut self) -> bool {
        self.accumulator += self.last_update.elapsed();
        self.last_update = Instant::now();

        if self.accumulator >= self.tick_duration {
            self.accumulator -= self.tick_duration;
            true
        } else {
            false
        }
    }
}
