extern crate log;
extern crate zerocopy;

use crate::game::Game;

#[macro_use]
pub mod helpers;
#[macro_use]
pub mod services;
pub mod block;
pub mod entity;
pub mod game;
pub mod render;
pub mod world;

fn main() {
    let game = Game::new();
    game.run();
}
