use crate::keyboard::KeyCode;

#[derive(Debug, Clone)]
pub enum EngineEvent {
    Init,
    Update(f32),
    Render,
    Resized(u32, u32),
    CloseRequested,
    KeyboardInput {
        key: KeyCode,
        pressed: bool,
        repeat: bool,
    },
    ModifiersChanged {
        shift: bool,
        control: bool,
        alt: bool,
        super_key: bool,
    },
    CursorMoved {
        x: f64,
        y: f64,
    },
}
