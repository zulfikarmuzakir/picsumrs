use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;


#[derive(Debug, Clone)]
pub struct ProgressTracker {
    bar: ProgressBar,
    start_time: std::time::Instant,
}

impl ProgressTracker {
    pub fn new(total: u64, message: &str) -> Self {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/bluu}] {pos}/{len} ({eta}) {bytes_per_sec}")
                .unwrap()
                .progress_chars("##-"),
        );
        bar.set_message(message.to_string());

        Self {
            bar,
            start_time: std::time::Instant::now(),
        }
    }

    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    pub fn set_message(&self, msg: &str) {
        self.bar.set_message(msg.to_string());
    }

    pub fn finish_with_message(&self, msg: &str) {
        self.bar.finish_with_message(msg.to_string());
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}
