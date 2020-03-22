use crate::game::game_state::GameState;
use specs::{DispatcherBuilder, WorldExt};
use crate::entity::ecs::physics::PhysicsSystem;
use crate::entity::ecs::movement::MovementSystem;

pub mod ecs;
pub mod player;

pub fn process_entities(game: &mut GameState) {

    let movement = MovementSystem {
        rotation: None,
        position: None,
        velocity: None
    };

    let mut dispatcher = DispatcherBuilder::new()
        .with(PhysicsSystem, "physics", &[])
        .with(movement, "update_pos", &["physics"])
        .build();

    dispatcher.dispatch(&mut game.store_world);
    game.store_world.maintain();
}