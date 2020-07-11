use crate::services::{input_service::input::GameChanges, ServicesContext};
use specs::World;
use crate::services::input_service::actions::ActionSheet;

pub mod input;
pub mod actions;

pub struct InputService {}

impl InputService {
    pub fn new(context: &ServicesContext, universe: &mut World) -> InputService {
        let changes = GameChanges::new(context.window.clone());
        let actionsheet = ActionSheet::new();

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
