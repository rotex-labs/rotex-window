use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::error::Error;

/// Live window operations available while an [`crate::EngineApp`] handles an event.
///
/// Also exposes [`HasWindowHandle`] and [`HasDisplayHandle`] so a renderer can
/// build a surface from the same object.
pub trait WindowBridge: HasWindowHandle + HasDisplayHandle {
    /// Set the window title.
    fn set_title(&self, title: &str);

    /// Show or hide the cursor while it is over the window.
    fn set_cursor_visible(&self, visible: bool);

    /// Confine the cursor to the window when `grab` is `true`.
    ///
    /// Releases the cursor when `grab` is `false`.
    fn set_cursor_grab(&self, grab: bool) -> Result<(), Error>;

    /// Ask the backend to deliver another [`crate::EngineEvent::Render`].
    fn request_redraw(&self);

    /// Inner size of the window in physical pixels.
    ///
    /// Each component is at least 1.
    fn extent(&self) -> (u32, u32);
}
