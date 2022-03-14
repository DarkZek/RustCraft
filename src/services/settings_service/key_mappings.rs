use winit::event::VirtualKeyCode;

/// Stores configurable key mappings
pub struct KeyMapping {
    pub forwards: VirtualKeyCode,
    pub backwards: VirtualKeyCode,
    pub left: VirtualKeyCode,
    pub right: VirtualKeyCode,
    pub pause: VirtualKeyCode,
    pub debugging: VirtualKeyCode,
    pub jump: VirtualKeyCode,
    pub sneak: VirtualKeyCode,
    pub ctrl: VirtualKeyCode,
}

impl KeyMapping {
    pub fn default() -> KeyMapping {
        KeyMapping {
            forwards: VirtualKeyCode::W,
            backwards: VirtualKeyCode::S,
            left: VirtualKeyCode::A,
            right: VirtualKeyCode::D,
            pause: VirtualKeyCode::Escape,
            debugging: VirtualKeyCode::F3,
            jump: VirtualKeyCode::Space,
            sneak: VirtualKeyCode::LShift,
            ctrl: VirtualKeyCode::LControl,
        }
    }
}
