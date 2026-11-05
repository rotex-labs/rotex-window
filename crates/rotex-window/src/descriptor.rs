/// Size, title, cursor, and presentation settings for a window a backend will create.
#[derive(Debug, Clone)]
pub struct WindowDescriptor {
    /// Text shown in the title bar.
    pub title: String,
    /// Requested width in logical pixels.
    pub width: u32,
    /// Requested height in logical pixels.
    pub height: u32,
    /// How the window occupies the screen.
    pub mode: WindowMode,
    /// Whether presentation should synchronize with the display refresh.
    ///
    /// A backend that does not present frames may ignore this flag.
    pub vsync: bool,
    /// Whether the cursor is shown over the window.
    pub cursor_visible: bool,
    /// Whether the cursor is confined to the window.
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

/// How a window occupies the screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    /// A decorated window at the requested size.
    Windowed,
    /// A borderless window covering the primary monitor.
    BorderlessFullscreen,
    /// Exclusive fullscreen on a monitor video mode when one is available.
    ExclusiveFullscreen,
}
