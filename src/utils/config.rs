use crate::logger::Logger;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub line_count: f32,
}

impl Config {
    pub fn new() -> Self {
        Self { line_count: 12.0 }
    }

    pub fn set_line_count(&mut self, line_count: f32) {
        self.line_count = line_count;
        Logger::info(format!("Set Line count to: {}", line_count));
    }
}
