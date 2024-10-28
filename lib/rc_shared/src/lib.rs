#![feature(const_for)]
#![feature(const_trait_impl)]
#![feature(sync_unsafe_cell)]

pub mod aabb;
pub mod atlas;
pub mod biome;
pub mod block;
pub mod chunk;
pub mod helpers;
pub mod item;
pub mod viewable_direction;
pub mod game_objects;
pub mod constants;
pub mod relative_chunk_map;
pub mod relative_chunk_flat_map;
pub mod game_mode;
pub mod chunk_column;
pub mod time;

pub const CHUNK_SIZE: usize = 16;

pub const MAX_LIGHT_VALUE: usize = 16;

pub const PHYSICS_SYNC_RATE_SECONDS: f64 = 1.0 / 20.0;