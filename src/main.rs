#![feature(once_cell)]
#![feature(trivial_bounds)]
#![feature(in_band_lifetimes)]
#![feature(async_closure)]

#[macro_use]
extern crate lazy_static;
extern crate zerocopy;
#[macro_use]
extern crate rc_logging;

extern crate serde;

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

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let game = Game::new();
    game.run();
}
