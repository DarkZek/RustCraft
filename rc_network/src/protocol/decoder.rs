/*
   This packet decoder is designed to be used only when the connection is in Play mode,
   and should be able to accept any incoming connections and store its information.
*/

use nbt::Blob;
use std::io::{Read, Error, Cursor};
use crate::{
    stream::NetworkStream,
    protocol::{
        data::read_types::{
            read_bool, read_bytearray, read_float, read_int, read_intarray, read_long, read_string,
            read_unsignedbyte, read_varint, read_varintarray,
        },
        types::chunk::NetworkChunk,
        types::{PVarType, PVarTypeTemplate},
        packet::{
            PacketType,
            PacketData,
            info::join_game::JoinGamePacket,
            info::server_difficulty::ServerDifficultyPacket,
            player::player_abilities::PlayerAbilitiesPacket,
            player::held_item_change::HeldItemChangePacket,
            inventory::declare_recipes::DeclareRecipesPacket,
            info::plugin_message::PluginMessagePacket,
            PacketData::{ServerDifficulty, DeclareRecipes, ChatMessage, UpdateLightLevels},
            info::tags::TagsPacket,
            entity::status::EntityStatusPacket,
            player::position_rotation::PlayerPositionRotationPacket,
            inventory::unlock_recipes::UnlockRecipesPacket,
            player::position_look::PlayerPositionLookPacket,
            info::chat_message::ChatMessagePacket,
            info::player_list_info::PlayerListInfoPacket,
            entity::update_metadata::EntityUpdateMetadataPacket,
            player::view_chunk_position::UpdateViewChunkPositionPacket,
            world::update_light::UpdateLightLevelsPacket
        }
    }
};
use std::io;
use crate::protocol::packet::world::chunk_data::ChunkDataPacket;
use crate::protocol::packet::entity::spawn_living_entity::SpawnLivingEntityPacket;
use crate::protocol::packet::entity::set_properties::EntitySetPropertiesPacket;
use crate::protocol::packet::entity::head_look::EntityHeadLookPacket;
use crate::protocol::packet::entity::equipment::EntityEquipmentPacket;
use crate::protocol::packet::entity::spawn_entity::SpawnEntityPacket;
use crate::protocol::packet::PacketData::{EntityVelocity, WindowItems, UpdatePlayerHealth};
use crate::protocol::packet::entity::velocity::EntityVelocityPacket;
use crate::protocol::packet::world::border::WorldBorderPacket;
use crate::protocol::packet::world::time_update::TimeUpdatePacket;
use crate::protocol::packet::world::spawn_position::SpawnPositionPacket;
use crate::protocol::packet::info::change_game_state::ChangeGameStatePacket;
use crate::protocol::packet::inventory::window_items::WindowItemsPacket;
use crate::protocol::packet::inventory::set_slot::SetSlotPacket;
use crate::protocol::packet::player::update_health::UpdatePlayerHealthPacket;
use crate::protocol::packet::player::set_experience::SetPlayerExperiencePacket;
use crate::protocol::packet::info::keep_alive::KeepAlivePacket;

pub struct PacketDecoder;

impl PacketDecoder {
    pub fn decode(stream: &mut NetworkStream) -> Result<PacketData, io::Error> {
        // Get length of packet
        let len = read_varint(stream);

        stream.reset_byte_counter();

        // Get id of packet
        let packet_id = read_varint(stream);
        let mut buf = vec![0; (len - stream.get_bytes_read() as i64) as usize];

        stream.read_exact(&mut buf)?;

        let mut cursor = Cursor::new(buf);

        let packet = match packet_id {
            0xe => PacketData::ServerDifficulty(*ServerDifficultyPacket::deserialize(&mut cursor)),
            0x19 => PacketData::PluginMessage(*PluginMessagePacket::deserialize(&mut cursor)),
            0x26 => PacketData::JoinGame(*JoinGamePacket::deserialize(&mut cursor)),
            0x32 => PacketData::PlayerAbilities(*PlayerAbilitiesPacket::deserialize(&mut cursor)),
            0x40 => PacketData::HeldItemChange(*HeldItemChangePacket::deserialize(&mut cursor)),
            0x5b => PacketData::DeclareRecipes(*DeclareRecipesPacket::deserialize(&mut cursor)),
            0x5c => PacketData::Tags(*TagsPacket::deserialize(&mut cursor)),
            0x1c => PacketData::EntityStatus(*EntityStatusPacket::deserialize(&mut cursor)),
            0x12 => PacketData::PlayerPositionRotation(*PlayerPositionRotationPacket::deserialize(&mut cursor)),
            0x37 => PacketData::UnlockRecipes(*UnlockRecipesPacket::deserialize(&mut cursor)),
            0x36 => PacketData::PlayerPositionLook(*PlayerPositionLookPacket::deserialize(&mut cursor)),
            0x0F => PacketData::ChatMessage(*ChatMessagePacket::deserialize(&mut cursor)),
            0x34 => PacketData::PlayerListInfo(*PlayerListInfoPacket::deserialize(&mut cursor)),
            0x44 => PacketData::EntityUpdateMetadata(*EntityUpdateMetadataPacket::deserialize(&mut cursor)),
            0x41 => PacketData::UpdateViewChunkPosition(*UpdateViewChunkPositionPacket::deserialize(&mut cursor)),
            0x25 => PacketData::UpdateLightLevels(*UpdateLightLevelsPacket::deserialize(&mut cursor)),
            0x22 => PacketData::ChunkData(*ChunkDataPacket::deserialize(&mut cursor)),
            0x03 => PacketData::SpawnLivingEntity(*SpawnLivingEntityPacket::deserialize(&mut cursor)),
            0x59 => PacketData::EntitySetProperties(*EntitySetPropertiesPacket::deserialize(&mut cursor)),
            0x3c => PacketData::EntityHeadLook(*EntityHeadLookPacket::deserialize(&mut cursor)),
            0x47 => PacketData::EntityEquipment(*EntityEquipmentPacket::deserialize(&mut cursor)),
            0x00 => PacketData::SpawnEntity(*SpawnEntityPacket::deserialize(&mut cursor)),
            0x46 => PacketData::EntityVelocity(*EntityVelocityPacket::deserialize(&mut cursor)),
            0x3e => PacketData::WorldBorder(*WorldBorderPacket::deserialize(&mut cursor)),
            0x4F => PacketData::TimeUpdate(*TimeUpdatePacket::deserialize(&mut cursor)),
            0x4E => PacketData::SpawnPosition(*SpawnPositionPacket::deserialize(&mut cursor)),
            0x1F => PacketData::ChangeGameState(*ChangeGameStatePacket::deserialize(&mut cursor)),
            0x15 => PacketData::WindowItems(*WindowItemsPacket::deserialize(&mut cursor)),
            0x17 => PacketData::SetSlot(*SetSlotPacket::deserialize(&mut cursor)),
            0x49 => PacketData::UpdatePlayerHealth(*UpdatePlayerHealthPacket::deserialize(&mut cursor)),
            0x48 => PacketData::SetPlayerExperience(*SetPlayerExperiencePacket::deserialize(&mut cursor)),
            0x21 => PacketData::KeepAlive(*KeepAlivePacket::deserialize(&mut cursor)),
            _ => panic!(format!("Unknown packet ID: 0x{:x}", packet_id))
        };

        if len - stream.get_bytes_read() as i64 > 0 {
            println!("Remaining Length: {}", len - stream.get_bytes_read() as i64);
        } else if (len - stream.get_bytes_read() as i64) < 0 {
            println!("Read {} too many bytes on packet type 0x{:x}", stream.get_bytes_read() as i64 - len, packet_id);
        }

        while len - stream.get_bytes_read() as i64 > 0 {
            read_unsignedbyte(stream);
        }

        Ok(packet)
    }
}