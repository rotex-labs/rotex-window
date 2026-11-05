use crate::{EngineEvent, WindowBridge};

/// Application driven by a [`crate::WindowBackend`].
///
/// The backend calls [`EngineApp::on_event`] once for each [`EngineEvent`].
pub trait EngineApp {
    /// Handle one engine event.
    ///
    /// `window` is the bridge for the window that produced `event`.
    fn on_event(&mut self, event: EngineEvent, window: &dyn WindowBridge);
}
