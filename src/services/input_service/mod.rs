use crate::services::input_service::actions::ActionSheet;
use crate::services::{input_service::input::InputState, ServicesContext};
use specs::World;

pub mod actions;
pub mod input;

pub struct InputService {}

impl InputService {
    pub fn new(context: &ServicesContext, universe: &mut World) -> InputService {
        let mut changes = InputState::new(context.window.clone());
        let actionsheet = ActionSheet::new();

        //changes.set_capture_mouse();

        universe.insert(changes);
        universe.insert(actionsheet);

        InputService {}
    }
}

impl Default for InputService {
    fn default() -> Self {
        unimplemented!()
    }
}
