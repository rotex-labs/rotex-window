use crate::keyboard::KeyCode;

/// Lifecycle and input events delivered to an [`crate::EngineApp`].
#[derive(Debug, Clone)]
pub enum EngineEvent {
    /// The window has been created and the application can initialize.
    Init,
    /// Time has advanced since the previous update.
    Update(
        /// Elapsed seconds since the previous update. Zero on the first update.
        f32,
    ),
    /// The backend is ready for the application to draw a frame.
    Render,
    /// The window's inner size changed.
    Resized(
        /// New width in physical pixels. At least 1.
        u32,
        /// New height in physical pixels. At least 1.
        u32,
    ),
    /// The user asked the window to close.
    CloseRequested,
    /// A physical key changed state.
    KeyboardInput {
        /// Physical key that changed.
        key: KeyCode,
        /// `true` when the key is pressed, `false` when it is released.
        pressed: bool,
        /// `true` when the event is a key repeat.
        repeat: bool,
    },
    /// The active modifier mask changed.
    ModifiersChanged {
        /// Shift is held.
        shift: bool,
        /// Control is held.
        control: bool,
        /// Alt is held.
        alt: bool,
        /// Super (Windows, Command, or Meta) is held.
        super_key: bool,
    },
    /// The cursor moved inside the window.
    CursorMoved {
        /// Horizontal position in physical pixels.
        x: f64,
        /// Vertical position in physical pixels.
        y: f64,
    },
}
