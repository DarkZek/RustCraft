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

pub const CHUNK_SIZE: usize = 16;

pub const MAX_LIGHT_VALUE: usize = 16;