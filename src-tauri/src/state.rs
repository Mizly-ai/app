use std::sync::Mutex;

/// 應用程式全局狀態
pub struct AppState {
    /// 是否防止自動隱藏視窗
    pub prevent_auto_hide: Mutex<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            prevent_auto_hide: Mutex::new(false),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
