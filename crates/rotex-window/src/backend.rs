use crate::{EngineApp, Error, WindowDescriptor};

pub trait WindowBackend {
    fn run(
        self: Box<Self>,
        descriptor: WindowDescriptor,
        app: Box<dyn EngineApp>,
    ) -> Result<(), Error>;
}
