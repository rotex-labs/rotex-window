use crate::{EngineApp, Error, WindowBackend, WindowDescriptor};

/// Owns a [`WindowBackend`] and starts the application loop.
pub struct WindowOrchestrator {
    backend: Box<dyn WindowBackend>,
}

impl WindowOrchestrator {
    /// Wrap a backend that will create the window and run the loop.
    pub fn new(backend: Box<dyn WindowBackend>) -> Self {
        Self { backend }
    }

    /// Create the window from `descriptor` and drive `app` until the loop ends.
    pub fn run(self, descriptor: WindowDescriptor, app: Box<dyn EngineApp>) -> Result<(), Error> {
        self.backend.run(descriptor, app)
    }
}
