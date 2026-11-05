#[derive(Debug, Clone)]
pub struct WindowDescriptor {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub mode: WindowMode,
    pub vsync: bool,
    pub cursor_visible: bool,
    pub cursor_grab: bool,
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        Self {
            title: "Rotex Window".to_string(),
            width: 1280,
            height: 720,
            mode: WindowMode::Windowed,
            vsync: true,
            cursor_visible: true,
            cursor_grab: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    ExclusiveFullscreen,
}
