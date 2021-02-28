use crate::protocol::data::read_types::{read_bool, read_string, read_uuid, read_varint};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct PlayerListInfoPacket {
    pub actions: Vec<(u128, PlayerListInfoUpdateAction)>,
}

impl PacketType for PlayerListInfoPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let action = read_varint(buf);
        let players = read_varint(buf);
        let mut actions = Vec::new();

        for _ in 0..players {
            let uuid = read_uuid(buf);

            match action {
                0 => {
                    let name = read_string(buf);
                    let mut properties = Vec::new();
                    let properties_count = read_varint(buf);

                    for _ in 0..properties_count {
                        let name = read_string(buf);
                        let value = read_string(buf);
                        let is_signed = read_bool(buf);
                        let signature = if is_signed {
                            Some(read_string(buf))
                        } else {
                            None
                        };

                        properties.push((name, value, is_signed, signature));
                    }

                    let gamemode = read_varint(buf);
                    let ping = read_varint(buf);
                    let has_display_name = read_bool(buf);
                    let display_name = if has_display_name {
                        Some(read_string(buf))
                    } else {
                        None
                    };

                    actions.push((
                        uuid,
                        PlayerListInfoUpdateAction::AddPlayer(
                            PlayerListInfoUpdateActionAddPlayer {
                                name,
                                properties,
                                gamemode,
                                ping,
                                has_display_name,
                                display_name,
                            },
                        ),
                    ))
                }
                1 => actions.push((
                    uuid,
                    PlayerListInfoUpdateAction::UpdateGamemode(read_varint(buf)),
                )),
                2 => actions.push((
                    uuid,
                    PlayerListInfoUpdateAction::UpdateLatency(read_varint(buf)),
                )),
                3 => {
                    let has_display_name = read_bool(buf);

                    let display_name = if has_display_name {
                        Some(read_string(buf))
                    } else {
                        None
                    };

                    actions.push((
                        uuid,
                        PlayerListInfoUpdateAction::UpdateDisplayName(
                            has_display_name,
                            display_name,
                        ),
                    ))
                }
                4 => actions.push((uuid, PlayerListInfoUpdateAction::RemovePlayer)),
                _ => {}
            }
        }

        Box::new(PlayerListInfoPacket { actions })
    }
}

#[derive(Debug)]
pub enum PlayerListInfoUpdateAction {
    AddPlayer(PlayerListInfoUpdateActionAddPlayer),
    UpdateGamemode(i64),
    UpdateLatency(i64),
    UpdateDisplayName(bool, Option<String>),
    RemovePlayer,
}

#[derive(Debug, Clone)]
pub struct PlayerListInfoUpdateActionAddPlayer {
    name: String,
    properties: Vec<(String, String, bool, Option<String>)>,
    gamemode: i64,
    ping: i64,
    has_display_name: bool,
    display_name: Option<String>,
}
