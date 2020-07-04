use crate::services::{
    ServicesContext,
    input_service::input::GameChanges
};
use specs::{World};

pub mod input;
pub mod systems;

pub struct InputService {
}

impl InputService {
    pub fn new(context: &ServicesContext, universe: &mut World) -> InputService {
        let changes = GameChanges::new(context.window.clone());

        universe.insert(changes);

        InputService {
        }
    }
}

impl Default for InputService {
    fn default() -> Self {
        unimplemented!()
    }
}