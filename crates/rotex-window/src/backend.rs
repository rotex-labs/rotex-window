use crate::{EngineApp, Error, WindowDescriptor};

/// Creates a window and runs an [`EngineApp`] until the loop ends.
pub trait WindowBackend {
    /// Open the window described by `descriptor` and drive `app`.
    ///
    /// Consumes the backend. Returns when the event loop exits, or earlier when
    /// the window cannot be created.
    fn run(
        self: Box<Self>,
        descriptor: WindowDescriptor,
        app: Box<dyn EngineApp>,
    ) -> Result<(), Error>;
}
