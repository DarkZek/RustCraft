use winit::event::{MouseButton, WindowEvent};
use winit::window::Window;
use crate::client::events::{GameChanges, GameChangesContext};
use winit::dpi::{PhysicalSize, PhysicalPosition};

impl GameChanges {
    pub fn new() -> GameChanges {
        GameChanges {
            movement: [0; 2],
            look: [0.0; 2],
            use_item: false,
            activate_item: false,
            pause: false,
            jump: false,
            sneak: false,
            mouse: None
        }
    }

    pub fn clear(&mut self) {
        *self = GameChanges::new();
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

    pub fn handle_event(&mut self, event: &WindowEvent, changes: &mut GameChangesContext, window: &Window) {
        match *event.clone() {
            WindowEvent::MouseInput { device_id: _, state: _, button, .. } => {
                if button == MouseButton::Left {
                    self.item_used();
                } else if button == MouseButton::Right {
                    self.item_activated();
                }

                if !changes.grabbed {
                    changes.grabbed = true;
                    capture_mouse(window);
                }
            }

            WindowEvent::KeyboardInput { device_id: _device_id, input, is_synthetic: _ } => {
                if input.virtual_keycode != None && changes.grabbed {
                    let key = input.virtual_keycode.unwrap();

                    if key == changes.mappings.pause {
                        self.pause_pressed();
                        changes.grabbed = false;
                        uncapture_mouse(window);
                    }

                    if key == changes.mappings.forwards {
                        self.add_forward_movement_changes(1);
                    }

                    if key == changes.mappings.backwards {
                        self.add_forward_movement_changes(-1);
                    }

                    if key == changes.mappings.left {
                        self.add_horizontal_movement_changes(1);
                    }

                    if key == changes.mappings.right {
                        self.add_horizontal_movement_changes(-1);
                    }

                    if key == changes.mappings.jump {
                        self.set_jump();
                    }

                    if key == changes.mappings.sneak {
                        self.set_sneak();
                    }
                }
            }

            WindowEvent::CursorMoved { device_id: _device_id, position, .. } => {

                self.cursor_position(position);

                if changes.grabbed {
                    let raw_x = position.x as f64;
                    let raw_y = position.y as f64;

                    let x = -1.0 * (raw_x - changes.mouse_home.x as f64);
                    let y = -1.0 * (raw_y - changes.mouse_home.y as f64);

                    self.look[0] += x;
                    self.look[1] += y;

                    if let Err(e) = window.set_cursor_position(changes.mouse_home) {
                        log_error!("Error setting cursor position: {}", e);
                    }
                }
            }
            _ => {}
        }
    }
}

fn capture_mouse(window: &Window) {
    if let Err(e) = window.set_cursor_grab(true) { println!("Error grabbing cursor: {}", e); }
    window.set_cursor_visible(false);
}

fn uncapture_mouse(window: &Window) {
    if let Err(e) = window.set_cursor_grab(false) { println!("Error releasing cursor: {}", e); }
    window.set_cursor_visible(true);
}