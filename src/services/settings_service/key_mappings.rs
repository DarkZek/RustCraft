use winit::event::VirtualKeyCode;

//TODO: Move this to settings service

/// Stores configurable key mappings
pub struct KeyMapping {
    pub forwards: VirtualKeyCode,
    pub backwards: VirtualKeyCode,
    pub left: VirtualKeyCode,
    pub right: VirtualKeyCode,
    pub pause: VirtualKeyCode,
    pub jump: VirtualKeyCode,
    pub sneak: VirtualKeyCode,
}

impl KeyMapping {
    pub fn default() -> KeyMapping {
        KeyMapping {
            forwards: VirtualKeyCode::W,
            backwards: VirtualKeyCode::S,
            left: VirtualKeyCode::A,
            right: VirtualKeyCode::D,
            pause: VirtualKeyCode::Escape,
            jump: VirtualKeyCode::Space,
            sneak: VirtualKeyCode::LShift,
        }
    }
}
