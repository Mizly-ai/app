//! Application configuration

/// Default window dimensions
pub const WINDOW_WIDTH: f64 = 700.0;
pub const WINDOW_HEIGHT: f64 = 500.0;

/// Application window configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window_width: f64,
    pub window_height: f64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
        }
    }
}
