use crate::protocol::packet::clientbound::effect::effect::EntityEffectPacket;
use crate::protocol::packet::clientbound::effect::play::PlayEffectPacket;
use crate::protocol::packet::clientbound::effect::sound::SoundEffectPacket;
use crate::protocol::packet::clientbound::entity::animation::EntityAnimationPacket;
use crate::protocol::packet::clientbound::entity::destroy_entities::DestroyEntitiesPacket;
use crate::protocol::packet::clientbound::entity::equipment::EntityEquipmentPacket;
use crate::protocol::packet::clientbound::entity::head_look::EntityHeadLookPacket;
use crate::protocol::packet::clientbound::entity::set_passengers::SetPassengersPacket;
use crate::protocol::packet::clientbound::entity::set_properties::EntitySetPropertiesPacket;
use crate::protocol::packet::clientbound::entity::spawn_entity::SpawnEntityPacket;
use crate::protocol::packet::clientbound::entity::spawn_living_entity::SpawnLivingEntityPacket;
use crate::protocol::packet::clientbound::entity::status::EntityStatusPacket;
use crate::protocol::packet::clientbound::entity::teleport::EntityTeleportPacket;
use crate::protocol::packet::clientbound::entity::update_metadata::EntityUpdateMetadataPacket;
use crate::protocol::packet::clientbound::entity::update_position::UpdateEntityPositionPacket;
use crate::protocol::packet::clientbound::entity::update_position_rotation::UpdateEntityPositionRotationPacket;
use crate::protocol::packet::clientbound::entity::update_rotation::UpdateEntityRotationPacket;
use crate::protocol::packet::clientbound::entity::velocity::EntityVelocityPacket;
use crate::protocol::packet::clientbound::info::advancements::AdvancementsPacket;
use crate::protocol::packet::clientbound::info::change_game_state::ChangeGameStatePacket;
use crate::protocol::packet::clientbound::info::chat_message::ChatMessagePacket;
use crate::protocol::packet::clientbound::info::disconnect::DisconnectPacket;
use crate::protocol::packet::clientbound::info::join_game::JoinGamePacket;
use crate::protocol::packet::clientbound::info::keep_alive::KeepAlivePacket;
use crate::protocol::packet::clientbound::info::player_list_info::PlayerListInfoPacket;
use crate::protocol::packet::clientbound::info::plugin_message::PluginMessagePacket;
use crate::protocol::packet::clientbound::info::server_difficulty::ServerDifficultyPacket;
use crate::protocol::packet::clientbound::info::tags::TagsPacket;
use crate::protocol::packet::clientbound::inventory::declare_recipes::DeclareRecipesPacket;
use crate::protocol::packet::clientbound::inventory::set_slot::SetSlotPacket;
use crate::protocol::packet::clientbound::inventory::unlock_recipes::UnlockRecipesPacket;
use crate::protocol::packet::clientbound::inventory::window_items::WindowItemsPacket;
use crate::protocol::packet::clientbound::player::held_item_change::HeldItemChangePacket;
use crate::protocol::packet::clientbound::player::login_success::LoginSuccessPacket;
use crate::protocol::packet::clientbound::player::player_abilities::PlayerAbilitiesPacket;
use crate::protocol::packet::clientbound::player::position_look::PlayerPositionLookPacket;
use crate::protocol::packet::clientbound::player::position_rotation::PlayerPositionRotationPacket;
use crate::protocol::packet::clientbound::player::set_experience::SetPlayerExperiencePacket;
use crate::protocol::packet::clientbound::player::spawn::SpawnPlayerPacket;
use crate::protocol::packet::clientbound::player::update_health::UpdatePlayerHealthPacket;
use crate::protocol::packet::clientbound::player::view_chunk_position::UpdateViewChunkPositionPacket;
use crate::protocol::packet::clientbound::world::block_change::BlockChangePacket;
use crate::protocol::packet::clientbound::world::border::WorldBorderPacket;
use crate::protocol::packet::clientbound::world::chunk_data::ChunkDataPacket;
use crate::protocol::packet::clientbound::world::multi_block_change::MultiBlockChangePacket;
use crate::protocol::packet::clientbound::world::spawn_position::SpawnPositionPacket;
use crate::protocol::packet::clientbound::world::spawn_weather_entity::SpawnWeatherEntityPacket;
use crate::protocol::packet::clientbound::world::time_update::TimeUpdatePacket;
use crate::protocol::packet::clientbound::world::update_light::UpdateLightLevelsPacket;
use std::io::Cursor;

pub mod effect;
pub mod entity;
pub mod info;
pub mod inventory;
pub mod player;
pub mod world;

pub trait ClientBoundPacketType {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self>;
}

#[derive(Debug)]
pub enum ClientBoundPacketData {
    JoinGame(JoinGamePacket),
    LoginSuccess(LoginSuccessPacket),
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
    EntityEffect(EntityEffectPacket),
}
