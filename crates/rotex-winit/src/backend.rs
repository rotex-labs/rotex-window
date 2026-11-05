use rotex_window::{EngineApp, Error as WindowError, WindowBackend, WindowDescriptor};

use crate::runner;

/// [`rotex_window::WindowBackend`] that runs the application on winit.
#[derive(Default)]
pub struct WinitBackend;

impl WindowBackend for WinitBackend {
    fn run(
        self: Box<Self>,
        descriptor: WindowDescriptor,
        app: Box<dyn EngineApp>,
    ) -> Result<(), WindowError> {
        let _ = self;
        runner::run(descriptor, app)
    }
}
