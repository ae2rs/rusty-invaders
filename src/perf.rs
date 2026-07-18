use std::{
    thread,
    time::{Duration, Instant},
};

pub struct FrameTimer {
    target_frame_time: Duration,
    frame_start: Instant,
    fps_window_start: Instant,
    frame_count: u32,
    displayed_fps: f64,
}

impl FrameTimer {
    pub fn new(target_frame_time: Duration) -> Self {
        let now = Instant::now();

        Self {
            target_frame_time,
            frame_start: now,
            fps_window_start: now,
            frame_count: 0,
            displayed_fps: 0.0,
        }
    }

    pub fn begin_frame(&mut self) {
        self.frame_start = Instant::now();
    }

    pub fn end_frame(&mut self) -> Option<f64> {
        self.limit_frame_rate();

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
    pub fn fps(&self) -> f64 {
        self.displayed_fps
    }

    fn limit_frame_rate(&self) {
        let elapsed = self.frame_start.elapsed();

        if elapsed < self.target_frame_time {
            thread::sleep(self.target_frame_time - elapsed);
        }
    }

    fn update_fps(&mut self) {
        self.frame_count += 1;

        let elapsed = self.fps_window_start.elapsed();

        if elapsed >= Duration::from_secs(1) {
            self.displayed_fps = self.frame_count as f64 / elapsed.as_secs_f64();

            self.frame_count = 0;
            self.fps_window_start = Instant::now();
        }
    }
}
