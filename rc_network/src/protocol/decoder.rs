/*
   This packet decoder is designed to be used only when the connection is in Play mode,
   and should be able to accept any incoming connections and store its information.
*/

use crate::protocol::packet::entity::equipment::EntityEquipmentPacket;
use crate::protocol::packet::entity::head_look::EntityHeadLookPacket;
use crate::protocol::packet::entity::set_properties::EntitySetPropertiesPacket;
use crate::protocol::packet::entity::spawn_entity::SpawnEntityPacket;
use crate::protocol::packet::entity::spawn_living_entity::SpawnLivingEntityPacket;
use crate::protocol::packet::world::chunk_data::ChunkDataPacket;
use crate::{
    protocol::{
        data::read_types::{
            read_bool, read_bytearray, read_float, read_int, read_intarray, read_long, read_string,
            read_unsignedbyte, read_varint, read_varintarray,
        },
        packet::{
            entity::status::EntityStatusPacket,
            entity::update_metadata::EntityUpdateMetadataPacket,
            info::chat_message::ChatMessagePacket,
            info::join_game::JoinGamePacket,
            info::player_list_info::PlayerListInfoPacket,
            info::plugin_message::PluginMessagePacket,
            info::server_difficulty::ServerDifficultyPacket,
            info::tags::TagsPacket,
            inventory::declare_recipes::DeclareRecipesPacket,
            inventory::unlock_recipes::UnlockRecipesPacket,
            player::held_item_change::HeldItemChangePacket,
            player::player_abilities::PlayerAbilitiesPacket,
            player::position_look::PlayerPositionLookPacket,
            player::position_rotation::PlayerPositionRotationPacket,
            player::view_chunk_position::UpdateViewChunkPositionPacket,
            world::update_light::UpdateLightLevelsPacket,
            PacketData,
            PacketData::{ChatMessage, DeclareRecipes, ServerDifficulty, UpdateLightLevels},
            PacketType,
        },
        types::chunk::NetworkChunk,
        types::{PVarType, PVarTypeTemplate},
    },
    stream::NetworkStream,
};
use std::io;
use std::io::{Cursor, Read};

use crate::protocol::packet::effect::sound::SoundEffectPacket;
use crate::protocol::packet::entity::animation::EntityAnimationPacket;
use crate::protocol::packet::entity::destroy_entities::DestroyEntitiesPacket;
use crate::protocol::packet::entity::set_passengers::SetPassengersPacket;
use crate::protocol::packet::entity::teleport::EntityTeleportPacket;
use crate::protocol::packet::entity::update_position::UpdateEntityPositionPacket;
use crate::protocol::packet::entity::update_position_rotation::UpdateEntityPositionRotationPacket;
use crate::protocol::packet::entity::update_rotation::UpdateEntityRotationPacket;
use crate::protocol::packet::entity::velocity::EntityVelocityPacket;
use crate::protocol::packet::info::advancements::AdvancementsPacket;
use crate::protocol::packet::info::change_game_state::ChangeGameStatePacket;
use crate::protocol::packet::info::disconnect::DisconnectPacket;
use crate::protocol::packet::info::keep_alive::KeepAlivePacket;
use crate::protocol::packet::inventory::set_slot::SetSlotPacket;
use crate::protocol::packet::inventory::window_items::WindowItemsPacket;
use crate::protocol::packet::player::set_experience::SetPlayerExperiencePacket;
use crate::protocol::packet::player::spawn::SpawnPlayerPacket;
use crate::protocol::packet::player::update_health::UpdatePlayerHealthPacket;
use crate::protocol::packet::world::block_change::BlockChangePacket;
use crate::protocol::packet::world::border::WorldBorderPacket;
use crate::protocol::packet::world::multi_block_change::MultiBlockChangePacket;
use crate::protocol::packet::world::spawn_position::SpawnPositionPacket;
use crate::protocol::packet::world::spawn_weather_entity::SpawnWeatherEntityPacket;
use crate::protocol::packet::world::time_update::TimeUpdatePacket;
use crate::protocol::packet::effect::play::PlayEffectPacket;


pub struct PacketDecoder;

#[cfg_attr(rustfmt, rustfmt_skip)]
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
            0x58 => PacketData::Advancements(*AdvancementsPacket::deserialize(&mut cursor)),
            0x1B => PacketData::Disconnect(*DisconnectPacket::deserialize(&mut cursor)),
            0x0C => PacketData::BlockChange(*BlockChangePacket::deserialize(&mut cursor)),
            0x10 => PacketData::MultiBlockChange(*MultiBlockChangePacket::deserialize(&mut cursor)),
            0x05 => PacketData::SpawnPlayer(*SpawnPlayerPacket::deserialize(&mut cursor)),
            0x4B => PacketData::SetPassengers(*SetPassengersPacket::deserialize(&mut cursor)),
            0x29 => PacketData::UpdateEntityPosition(*UpdateEntityPositionPacket::deserialize(&mut cursor)),
            0x2B => PacketData::UpdateEntityRotation(*UpdateEntityRotationPacket::deserialize(&mut cursor)),
            0x2A => PacketData::UpdateEntityPositionRotation(*UpdateEntityPositionRotationPacket::deserialize(&mut cursor)),
            0x57 => PacketData::EntityTeleport(*EntityTeleportPacket::deserialize(&mut cursor)),
            0x02 => PacketData::SpawnWeatherEntity(*SpawnWeatherEntityPacket::deserialize(&mut cursor)),
            0x38 => PacketData::DestroyEntities(*DestroyEntitiesPacket::deserialize(&mut cursor)),
            0x52 => PacketData::SoundEffect(*SoundEffectPacket::deserialize(&mut cursor)),
            0x06 => PacketData::EntityAnimation(*EntityAnimationPacket::deserialize(&mut cursor)),
            0x23 => PacketData::PlayEffect(*PlayEffectPacket::deserialize(&mut cursor)),
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
