use std::collections::HashMap;
use bevy_testing_protocol::protocol::serverbound::authenticate::UserAuthenticate;
use naia_bevy_server::{RoomKey, UserKey};
use nalgebra::Vector3;
use crate::game::chunk::ChunkData;
use crate::game::player::Player;

pub struct Global {
    pub main_room_key: RoomKey,
    pub user_to_prediction_map: HashMap<UserKey, Player>,
    pub authentication_requests: HashMap<UserKey, String>,
    pub chunks: HashMap<Vector3<i32>, ChunkData>
}

impl Global {
    pub fn new(main_room_key: RoomKey) -> Self {

        let mut chunks = HashMap::new();

        for x in -1..=1 {
            for z in -1..=1 {
                let mut chunk = ChunkData::generate(Vector3::new(x, 0, z));
                chunks.insert(Vector3::new(x, 0, z), chunk);
            }
        }

        Global {
            main_room_key,
            user_to_prediction_map: Default::default(),
            authentication_requests: Default::default(),
            chunks
        }
    }
}