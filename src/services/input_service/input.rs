use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{MouseButton, WindowEvent};
use winit::window::Window;
use crate::services::settings_service::key_mappings::KeyMapping;
use std::sync::Arc;
use std::borrow::Borrow;

/// Tracks user input_service's since the last frame.
/// Naming them things like movement instead of WASD keys makes it easier to support multiple input_service device types.
pub struct GameChanges {
    pub movement: [i32; 2],
    pub look: [f64; 2],
    pub use_item: bool,
    pub activate_item: bool,
    pub pause: bool,
    pub jump: bool,
    pub sneak: bool,
    pub mouse: Option<PhysicalPosition<f64>>,

    pub mappings: KeyMapping,
    pub mouse_home: PhysicalPosition<u32>,
    pub grabbed: bool,
    window: Arc<Window>
}

impl GameChanges {
    pub fn new(window: Arc<Window>) -> GameChanges {
        GameChanges {
            movement: [0; 2],
            look: [0.0; 2],
            use_item: false,
            activate_item: false,
            pause: false,
            jump: false,
            sneak: false,
            mouse: None,
            mappings: KeyMapping::default(),
            mouse_home: PhysicalPosition::new(0, 0),
            grabbed: false,
            window
        }
    }

    pub fn clear(&mut self) {
        //TODO
        self.movement = [0; 2];
        self.look = [0.0; 2];
        self.use_item = false;
        self.activate_item = false;
        self.pause = false;
        self.jump = false;
        self.sneak = false;
        self.mouse = None;
    }

    fn set_jump(&mut self) {
        self.jump = true;
    }

    fn set_sneak(&mut self) {
        self.sneak = true;
    }

    fn add_forward_movement_changes(&mut self, change: i32) {
        self.movement[0] += change;
    }

    fn add_horizontal_movement_changes(&mut self, change: i32) {
        self.movement[1] += change;
    }

    fn item_used(&mut self) {
        self.use_item = true;
    }

    fn item_activated(&mut self) {
        self.activate_item = true;
    }

    fn pause_pressed(&mut self) {
        self.pause = true;
    }

    fn cursor_position(&mut self, new: PhysicalPosition<f64>) {
        self.mouse = Some(new);
    }

    pub fn resized(&mut self, size: &PhysicalSize<u32>) {
        self.mouse_home = PhysicalPosition {
            x: size.width / 2,
            y: size.height / 2
        };
    }

    //TODO: Eventually move this into a separate class so its easier to hook in controller game_changes

    /// Converts keyboard input_service game_changes into the different actions they perform.
    pub fn handle_event(
        &mut self,
        event: &WindowEvent,
    ) {
        match *event.clone() {
            WindowEvent::MouseInput {
                device_id: _,
                state: _,
                button,
                ..
            } => {
                if button == MouseButton::Left {
                    self.item_used();
                } else if button == MouseButton::Right {
                    self.item_activated();
                }

                if !self.grabbed {
                    self.grabbed = true;
                    capture_mouse(self.window.borrow());
                }
            }

            WindowEvent::KeyboardInput {
                device_id: _device_id,
                input,
                is_synthetic: _,
            } => {
                if input.virtual_keycode != None && self.grabbed {
                    let key = input.virtual_keycode.unwrap();

                    if key == self.mappings.pause {
                        self.pause_pressed();
                        self.grabbed = false;
                        uncapture_mouse(&*self.window.borrow());
                    }

                    if key == self.mappings.forwards {
                        self.add_forward_movement_changes(1);
                    }

                    if key == self.mappings.backwards {
                        self.add_forward_movement_changes(-1);
                    }

                    if key == self.mappings.left {
                        self.add_horizontal_movement_changes(1);
                    }

                    if key == self.mappings.right {
                        self.add_horizontal_movement_changes(-1);
                    }

                    if key == self.mappings.jump {
                        self.set_jump();
                    }

                    if key == self.mappings.sneak {
                        self.set_sneak();
                    }
                }
            }

            WindowEvent::CursorMoved {
                device_id: _device_id,
                position,
                ..
            } => {
                self.cursor_position(position);

                if self.grabbed {
                    let raw_x = position.x as f64;
                    let raw_y = position.y as f64;

                    let x = -1.0 * (raw_x - self.mouse_home.x as f64);
                    let y = -1.0 * (raw_y - self.mouse_home.y as f64);

                    self.look[0] += x;
                    self.look[1] += y;

                    if let Err(e) = (self.window.borrow() as &Window).set_cursor_position(self.mouse_home) {
                        log_error!("Error setting cursor position: {}", e);
                    }
                }
            }
            _ => {}
        }
    }
}

impl<'a> Default for GameChanges {
    fn default() -> Self {
        unimplemented!()
    }
}

fn capture_mouse(window: &Window) {
    if let Err(e) = window.set_cursor_grab(true) {
        println!("Error grabbing cursor: {}", e);
    }
    window.set_cursor_visible(false);
}

fn uncapture_mouse(window: &Window) {
    if let Err(e) = window.set_cursor_grab(false) {
        println!("Error releasing cursor: {}", e);
    }
    window.set_cursor_visible(true);
}
