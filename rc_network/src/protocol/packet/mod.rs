use crate::protocol::packet::effect::sound::SoundEffectPacket;
use crate::protocol::packet::entity::animation::EntityAnimationPacket;
use crate::protocol::packet::entity::destroy_entities::DestroyEntitiesPacket;
use crate::protocol::packet::entity::equipment::EntityEquipmentPacket;
use crate::protocol::packet::entity::head_look::EntityHeadLookPacket;
use crate::protocol::packet::entity::set_passengers::SetPassengersPacket;
use crate::protocol::packet::entity::set_properties::EntitySetPropertiesPacket;
use crate::protocol::packet::entity::spawn_entity::SpawnEntityPacket;
use crate::protocol::packet::entity::spawn_living_entity::SpawnLivingEntityPacket;
use crate::protocol::packet::entity::status::EntityStatusPacket;
use crate::protocol::packet::entity::teleport::EntityTeleportPacket;
use crate::protocol::packet::entity::update_metadata::EntityUpdateMetadataPacket;
use crate::protocol::packet::entity::update_position::UpdateEntityPositionPacket;
use crate::protocol::packet::entity::update_position_rotation::UpdateEntityPositionRotationPacket;
use crate::protocol::packet::entity::update_rotation::UpdateEntityRotationPacket;
use crate::protocol::packet::entity::velocity::EntityVelocityPacket;
use crate::protocol::packet::info::advancements::AdvancementsPacket;
use crate::protocol::packet::info::change_game_state::ChangeGameStatePacket;
use crate::protocol::packet::info::chat_message::ChatMessagePacket;
use crate::protocol::packet::info::disconnect::DisconnectPacket;
use crate::protocol::packet::info::join_game::JoinGamePacket;
use crate::protocol::packet::info::keep_alive::KeepAlivePacket;
use crate::protocol::packet::info::player_list_info::PlayerListInfoPacket;
use crate::protocol::packet::info::plugin_message::PluginMessagePacket;
use crate::protocol::packet::info::server_difficulty::ServerDifficultyPacket;
use crate::protocol::packet::info::tags::TagsPacket;
use crate::protocol::packet::inventory::declare_recipes::DeclareRecipesPacket;
use crate::protocol::packet::inventory::set_slot::SetSlotPacket;
use crate::protocol::packet::inventory::unlock_recipes::UnlockRecipesPacket;
use crate::protocol::packet::inventory::window_items::WindowItemsPacket;
use crate::protocol::packet::player::held_item_change::HeldItemChangePacket;
use crate::protocol::packet::player::player_abilities::PlayerAbilitiesPacket;
use crate::protocol::packet::player::position_look::PlayerPositionLookPacket;
use crate::protocol::packet::player::position_rotation::PlayerPositionRotationPacket;
use crate::protocol::packet::player::set_experience::SetPlayerExperiencePacket;
use crate::protocol::packet::player::spawn::SpawnPlayerPacket;
use crate::protocol::packet::player::update_health::UpdatePlayerHealthPacket;
use crate::protocol::packet::player::view_chunk_position::UpdateViewChunkPositionPacket;
use crate::protocol::packet::world::block_change::BlockChangePacket;
use crate::protocol::packet::world::border::WorldBorderPacket;
use crate::protocol::packet::world::chunk_data::ChunkDataPacket;
use crate::protocol::packet::world::multi_block_change::MultiBlockChangePacket;
use crate::protocol::packet::world::spawn_position::SpawnPositionPacket;
use crate::protocol::packet::world::spawn_weather_entity::SpawnWeatherEntityPacket;
use crate::protocol::packet::world::time_update::TimeUpdatePacket;
use crate::protocol::packet::world::update_light::UpdateLightLevelsPacket;
use std::io::Cursor;
use crate::protocol::packet::effect::play::PlayEffectPacket;

pub mod effect;
pub mod entity;
pub mod info;
pub mod inventory;
pub mod player;
pub mod world;

pub trait PacketType {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self>;
}

#[derive(Debug)]
pub enum PacketData {
    JoinGame(JoinGamePacket),
    PluginMessage(PluginMessagePacket),
    ServerDifficulty(ServerDifficultyPacket),
    PlayerAbilities(PlayerAbilitiesPacket),
    HeldItemChange(HeldItemChangePacket),
    DeclareRecipes(DeclareRecipesPacket),
    Tags(TagsPacket),
    EntityStatus(EntityStatusPacket),
    PlayerPositionRotation(PlayerPositionRotationPacket),
    UnlockRecipes(UnlockRecipesPacket),
    PlayerPositionLook(PlayerPositionLookPacket),
    ChatMessage(ChatMessagePacket),
    PlayerListInfo(PlayerListInfoPacket),
    EntityUpdateMetadata(EntityUpdateMetadataPacket),
    UpdateViewChunkPosition(UpdateViewChunkPositionPacket),
    UpdateLightLevels(UpdateLightLevelsPacket),
    ChunkData(ChunkDataPacket),
    SpawnLivingEntity(SpawnLivingEntityPacket),
    EntitySetProperties(EntitySetPropertiesPacket),
    EntityHeadLook(EntityHeadLookPacket),
    EntityEquipment(EntityEquipmentPacket),
    SpawnEntity(SpawnEntityPacket),
    EntityVelocity(EntityVelocityPacket),
    WorldBorder(WorldBorderPacket),
    TimeUpdate(TimeUpdatePacket),
    SpawnPosition(SpawnPositionPacket),
    ChangeGameState(ChangeGameStatePacket),
    WindowItems(WindowItemsPacket),
    SetSlot(SetSlotPacket),
    UpdatePlayerHealth(UpdatePlayerHealthPacket),
    SetPlayerExperience(SetPlayerExperiencePacket),
    KeepAlive(KeepAlivePacket),
    Advancements(AdvancementsPacket),
    Disconnect(DisconnectPacket),
    BlockChange(BlockChangePacket),
    MultiBlockChange(MultiBlockChangePacket),
    SpawnPlayer(SpawnPlayerPacket),
    SetPassengers(SetPassengersPacket),
    UpdateEntityPosition(UpdateEntityPositionPacket),
    UpdateEntityRotation(UpdateEntityRotationPacket),
    UpdateEntityPositionRotation(UpdateEntityPositionRotationPacket),
    EntityTeleport(EntityTeleportPacket),
    SpawnWeatherEntity(SpawnWeatherEntityPacket),
    DestroyEntities(DestroyEntitiesPacket),
    SoundEffect(SoundEffectPacket),
    EntityAnimation(EntityAnimationPacket),
    PlayEffect(PlayEffectPacket),
}
