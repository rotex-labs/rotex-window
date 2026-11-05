use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::error::Error;

pub trait WindowBridge: HasWindowHandle + HasDisplayHandle {
    fn set_title(&self, title: &str);

    fn set_cursor_visible(&self, visible: bool);

    fn set_cursor_grab(&self, grab: bool) -> Result<(), Error>;

    fn request_redraw(&self);

    fn extent(&self) -> (u32, u32);
}
