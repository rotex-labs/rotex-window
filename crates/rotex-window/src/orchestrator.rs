use crate::{EngineApp, Error, WindowBackend, WindowDescriptor};

pub struct WindowOrchestrator {
    backend: Box<dyn WindowBackend>,
}

impl WindowOrchestrator {
    pub fn new(backend: Box<dyn WindowBackend>) -> Self {
        Self { backend }
    }

    pub fn run(self, descriptor: WindowDescriptor, app: Box<dyn EngineApp>) -> Result<(), Error> {
        self.backend.run(descriptor, app)
    }
}
