#![feature(get_mut_unchecked)]
#![feature(fixed_size_array)]
#![feature(clamp)]

extern crate log;
extern crate shaderc;
extern crate zerocopy;

use crate::game::Game;

#[macro_use]
pub mod services;
pub mod block;
pub mod entity;
pub mod game;
pub mod helpers;
pub mod render;
pub mod world;

fn main() {
    let game = Game::new();
    game.run();
}
