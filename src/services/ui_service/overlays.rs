use crate::services::input_service::actions::ActionSheet;
use crate::services::input_service::input::InputState;
use crate::services::ui_service::components::UIComponents;
use crate::services::ui_service::UIService;
use specs::{Read, System, Write};

pub struct MenuOverlaySystem;

impl<'a> System<'a> for MenuOverlaySystem {
    type SystemData = (
        Read<'a, ActionSheet>,
        Read<'a, UIComponents>,
        Write<'a, InputState>,
    );

    fn run(&mut self, (sheet, ui_service, mut input_state): Self::SystemData) {
        if sheet.get_pause() {
            let mut pause_screen = ui_service.pause_menu_component.lock().unwrap();
            pause_screen.visible = !pause_screen.visible;
            if pause_screen.visible {
                input_state.set_uncapture_mouse();
            } else {
                input_state.set_capture_mouse();
            }
        }

        if sheet.get_debugging() {
            let mut pause_screen = ui_service.debug_screen_component.lock().unwrap();
            pause_screen.enabled = !pause_screen.enabled;
        }
    }
}
