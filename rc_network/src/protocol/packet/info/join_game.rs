use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_int, read_unsignedbyte, read_long, read_bool, read_varint, read_string};
use std::io::{Cursor, Seek};

#[derive(Debug)]
pub struct JoinGamePacket {
    pub entity_id: i32,
    pub gamemode: u8,
    pub dimension: i32,
    pub seed: i64,
    pub max_players: u8,
    pub level_type: String,
    pub view_distance: i64,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
}

impl PacketType for JoinGamePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_int(buf);
        let gamemode = read_unsignedbyte(buf);
        let dimension = read_int(buf);
        let seed = read_long(buf);
        let max_players = read_unsignedbyte(buf);
        let level_type = read_string(buf);
        let view_distance = read_varint(buf);
        let reduced_debug_info = read_bool(buf);
        let enable_respawn_screen = read_bool(buf);

        Box::new(JoinGamePacket {
            entity_id,
            gamemode,
            dimension,
            seed,
            max_players,
            level_type,
            view_distance,
            reduced_debug_info,
            enable_respawn_screen
        })
    }
}
