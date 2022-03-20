/*
   This packet decoder is designed to be used only when the connection is in Play mode,
   and should be able to accept any incoming connections and store its information.
*/

use crate::protocol::packet::clientbound::entity::equipment::EntityEquipmentPacket;
use crate::protocol::packet::clientbound::entity::head_look::EntityHeadLookPacket;
use crate::protocol::packet::clientbound::entity::set_properties::EntitySetPropertiesPacket;
use crate::protocol::packet::clientbound::entity::spawn_entity::SpawnEntityPacket;
use crate::protocol::packet::clientbound::entity::spawn_living_entity::SpawnLivingEntityPacket;
use crate::protocol::packet::clientbound::world::chunk_data::ChunkDataPacket;
use crate::{
    protocol::{
        data::read_types::{read_unsignedbyte, read_varint},
        packet::clientbound::{
            entity::status::EntityStatusPacket,
            entity::update_metadata::EntityUpdateMetadataPacket,
            info::chat_message::ChatMessagePacket, info::join_game::JoinGamePacket,
            info::player_list_info::PlayerListInfoPacket,
            info::plugin_message::PluginMessagePacket,
            info::server_difficulty::ServerDifficultyPacket, info::tags::TagsPacket,
            inventory::declare_recipes::DeclareRecipesPacket,
            inventory::unlock_recipes::UnlockRecipesPacket,
            player::held_item_change::HeldItemChangePacket,
            player::player_abilities::PlayerAbilitiesPacket,
            player::position_look::PlayerPositionLookPacket,
            player::position_rotation::PlayerPositionRotationPacket,
            player::view_chunk_position::UpdateViewChunkPositionPacket,
            world::update_light::UpdateLightLevelsPacket, ClientBoundPacketData,
            ClientBoundPacketType,
        },
    },
    stream::NetworkStream,
    ConnectionState,
};
use std::io;
use std::io::{Cursor, Read};

use crate::protocol::packet::clientbound::effect::effect::EntityEffectPacket;
use crate::protocol::packet::clientbound::effect::play::PlayEffectPacket;
use crate::protocol::packet::clientbound::effect::sound::SoundEffectPacket;
use crate::protocol::packet::clientbound::entity::animation::EntityAnimationPacket;
use crate::protocol::packet::clientbound::entity::destroy_entities::DestroyEntitiesPacket;
use crate::protocol::packet::clientbound::entity::set_passengers::SetPassengersPacket;
use crate::protocol::packet::clientbound::entity::teleport::EntityTeleportPacket;
use crate::protocol::packet::clientbound::entity::update_position::UpdateEntityPositionPacket;
use crate::protocol::packet::clientbound::entity::update_position_rotation::UpdateEntityPositionRotationPacket;
use crate::protocol::packet::clientbound::entity::update_rotation::UpdateEntityRotationPacket;
use crate::protocol::packet::clientbound::entity::velocity::EntityVelocityPacket;
use crate::protocol::packet::clientbound::info::advancements::AdvancementsPacket;
use crate::protocol::packet::clientbound::info::change_game_state::ChangeGameStatePacket;
use crate::protocol::packet::clientbound::info::disconnect::DisconnectPacket;
use crate::protocol::packet::clientbound::info::keep_alive::KeepAlivePacket;
use crate::protocol::packet::clientbound::inventory::set_slot::SetSlotPacket;
use crate::protocol::packet::clientbound::inventory::window_items::WindowItemsPacket;
use crate::protocol::packet::clientbound::player::login_success::LoginSuccessPacket;
use crate::protocol::packet::clientbound::player::set_experience::SetPlayerExperiencePacket;
use crate::protocol::packet::clientbound::player::spawn::SpawnPlayerPacket;
use crate::protocol::packet::clientbound::player::update_health::UpdatePlayerHealthPacket;
use crate::protocol::packet::clientbound::world::block_change::BlockChangePacket;
use crate::protocol::packet::clientbound::world::border::WorldBorderPacket;
use crate::protocol::packet::clientbound::world::multi_block_change::MultiBlockChangePacket;
use crate::protocol::packet::clientbound::world::spawn_position::SpawnPositionPacket;
use crate::protocol::packet::clientbound::world::spawn_weather_entity::SpawnWeatherEntityPacket;
use crate::protocol::packet::clientbound::world::time_update::TimeUpdatePacket;

pub struct PacketDecoder;

#[cfg_attr(rustfmt, rustfmt_skip)]
impl PacketDecoder {
    pub fn decode(stream: &mut NetworkStream, state: &mut ConnectionState) -> Result<ClientBoundPacketData, io::Error> {
        // Get length of packet
        let len = read_varint(stream);

        stream.reset_byte_counter();

        // Get id of packet
        let packet_id = read_varint(stream);
        let mut buf = vec![0; (len - stream.get_bytes_read() as i32) as usize];

        stream.read_exact(&mut buf)?;

        let mut cursor = Cursor::new(buf);
        
        if *state == ConnectionState::Connecting {
            match packet_id {
                0x02 => {
                    return Ok(ClientBoundPacketData::LoginSuccess(*LoginSuccessPacket::deserialize(&mut cursor)));
                }
                _ => {}
            }
        }

        let packet = match packet_id {
            0xe => ClientBoundPacketData::ServerDifficulty(*ServerDifficultyPacket::deserialize(&mut cursor)),
            0x19 => ClientBoundPacketData::PluginMessage(*PluginMessagePacket::deserialize(&mut cursor)),
            0x26 => ClientBoundPacketData::JoinGame(*JoinGamePacket::deserialize(&mut cursor)),
            0x32 => ClientBoundPacketData::PlayerAbilities(*PlayerAbilitiesPacket::deserialize(&mut cursor)),
            0x40 => ClientBoundPacketData::HeldItemChange(*HeldItemChangePacket::deserialize(&mut cursor)),
            0x5b => ClientBoundPacketData::DeclareRecipes(*DeclareRecipesPacket::deserialize(&mut cursor)),
            0x5c => ClientBoundPacketData::Tags(*TagsPacket::deserialize(&mut cursor)),
            0x1c => ClientBoundPacketData::EntityStatus(*EntityStatusPacket::deserialize(&mut cursor)),
            0x12 => ClientBoundPacketData::PlayerPositionRotation(*PlayerPositionRotationPacket::deserialize(&mut cursor)),
            0x37 => ClientBoundPacketData::UnlockRecipes(*UnlockRecipesPacket::deserialize(&mut cursor)),
            0x36 => ClientBoundPacketData::PlayerPositionLook(*PlayerPositionLookPacket::deserialize(&mut cursor)),
            0x0F => ClientBoundPacketData::ChatMessage(*ChatMessagePacket::deserialize(&mut cursor)),
            0x34 => ClientBoundPacketData::PlayerListInfo(*PlayerListInfoPacket::deserialize(&mut cursor)),
            0x44 => ClientBoundPacketData::EntityUpdateMetadata(*EntityUpdateMetadataPacket::deserialize(&mut cursor)),
            0x41 => ClientBoundPacketData::UpdateViewChunkPosition(*UpdateViewChunkPositionPacket::deserialize(&mut cursor)),
            0x25 => ClientBoundPacketData::UpdateLightLevels(*UpdateLightLevelsPacket::deserialize(&mut cursor)),
            0x22 => ClientBoundPacketData::ChunkData(*ChunkDataPacket::deserialize(&mut cursor)),
            0x03 => ClientBoundPacketData::SpawnLivingEntity(*SpawnLivingEntityPacket::deserialize(&mut cursor)),
            0x59 => ClientBoundPacketData::EntitySetProperties(*EntitySetPropertiesPacket::deserialize(&mut cursor)),
            0x3c => ClientBoundPacketData::EntityHeadLook(*EntityHeadLookPacket::deserialize(&mut cursor)),
            0x47 => ClientBoundPacketData::EntityEquipment(*EntityEquipmentPacket::deserialize(&mut cursor)),
            0x00 => ClientBoundPacketData::SpawnEntity(*SpawnEntityPacket::deserialize(&mut cursor)),
            0x46 => ClientBoundPacketData::EntityVelocity(*EntityVelocityPacket::deserialize(&mut cursor)),
            0x3e => ClientBoundPacketData::WorldBorder(*WorldBorderPacket::deserialize(&mut cursor)),
            0x4F => ClientBoundPacketData::TimeUpdate(*TimeUpdatePacket::deserialize(&mut cursor)),
            0x4E => ClientBoundPacketData::SpawnPosition(*SpawnPositionPacket::deserialize(&mut cursor)),
            0x1F => ClientBoundPacketData::ChangeGameState(*ChangeGameStatePacket::deserialize(&mut cursor)),
            0x15 => ClientBoundPacketData::WindowItems(*WindowItemsPacket::deserialize(&mut cursor)),
            0x17 => ClientBoundPacketData::SetSlot(*SetSlotPacket::deserialize(&mut cursor)),
            0x49 => ClientBoundPacketData::UpdatePlayerHealth(*UpdatePlayerHealthPacket::deserialize(&mut cursor)),
            0x48 => ClientBoundPacketData::SetPlayerExperience(*SetPlayerExperiencePacket::deserialize(&mut cursor)),
            0x21 => ClientBoundPacketData::KeepAlive(*KeepAlivePacket::deserialize(&mut cursor)),
            0x58 => ClientBoundPacketData::Advancements(*AdvancementsPacket::deserialize(&mut cursor)),
            0x1B => ClientBoundPacketData::Disconnect(*DisconnectPacket::deserialize(&mut cursor)),
            0x0C => ClientBoundPacketData::BlockChange(*BlockChangePacket::deserialize(&mut cursor)),
            0x10 => ClientBoundPacketData::MultiBlockChange(*MultiBlockChangePacket::deserialize(&mut cursor)),
            0x05 => ClientBoundPacketData::SpawnPlayer(*SpawnPlayerPacket::deserialize(&mut cursor)),
            0x4B => ClientBoundPacketData::SetPassengers(*SetPassengersPacket::deserialize(&mut cursor)),
            0x29 => ClientBoundPacketData::UpdateEntityPosition(*UpdateEntityPositionPacket::deserialize(&mut cursor)),
            0x2B => ClientBoundPacketData::UpdateEntityRotation(*UpdateEntityRotationPacket::deserialize(&mut cursor)),
            0x2A => ClientBoundPacketData::UpdateEntityPositionRotation(*UpdateEntityPositionRotationPacket::deserialize(&mut cursor)),
            0x57 => ClientBoundPacketData::EntityTeleport(*EntityTeleportPacket::deserialize(&mut cursor)),
            0x02 => ClientBoundPacketData::SpawnWeatherEntity(*SpawnWeatherEntityPacket::deserialize(&mut cursor)),
            0x38 => ClientBoundPacketData::DestroyEntities(*DestroyEntitiesPacket::deserialize(&mut cursor)),
            0x52 => ClientBoundPacketData::SoundEffect(*SoundEffectPacket::deserialize(&mut cursor)),
            0x06 => ClientBoundPacketData::EntityAnimation(*EntityAnimationPacket::deserialize(&mut cursor)),
            0x23 => ClientBoundPacketData::PlayEffect(*PlayEffectPacket::deserialize(&mut cursor)),
            0x5A => ClientBoundPacketData::EntityEffect(*EntityEffectPacket::deserialize(&mut cursor)),
            _ => panic!("Unknown packet ID: 0x{:x}", packet_id)
        };

        if len - stream.get_bytes_read() as i32 > 0 {
            log!("Remaining Length: {}", len - stream.get_bytes_read() as i32);
        } else if (len - stream.get_bytes_read() as i32) < 0 {
            log_error!("{}", format!("Read {} too many bytes on packet type 0x{:x}", stream.get_bytes_read() as i32 - len, packet_id));
        }

        while len - stream.get_bytes_read() as i32 > 0 {
            read_unsignedbyte(stream);
        }

        Ok(packet)
    }
}
