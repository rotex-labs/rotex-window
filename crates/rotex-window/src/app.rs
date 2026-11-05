use crate::{EngineEvent, WindowBridge};

pub trait EngineApp {
    fn on_event(&mut self, event: EngineEvent, window: &dyn WindowBridge);
}
