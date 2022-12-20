use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::client::Client;
use crate::server::Server;
use crate::config::Channel;
use crate::constants::{RawChunkData}; // todo: replace
use crate::messaging::serialize::make_serializer;
use crate::impl_message;
use crate::messaging::NetworkEntity;

make_serializer![
    BlockUpdate
    ChatSent
    FullChunkUpdate
    DespawnEntity
    EntityMoved
    EntityRotated
    SpawnEntity
    RequestChunk
];

#[derive(Serialize, Deserialize)]
pub struct BlockUpdate {
    pub id: u32,
    pub pos: IVec3,
}
impl_message!(BlockUpdate, 10, Channel::RELIABLE);

#[derive(Serialize, Deserialize)]
pub struct ChatSent {
    pub message: String,
}
impl_message!(ChatSent, 11, Channel::RELIABLE);

#[derive(Serialize, Deserialize)]
pub struct FullChunkUpdate {
    pub data: RawChunkData,
    pub pos: IVec3,
}
impl_message!(FullChunkUpdate, 12, Channel::CHUNK);

#[derive(Serialize, Deserialize)]
pub struct DespawnEntity {
    pub entity: NetworkEntity,
}
impl_message!(DespawnEntity, 13, Channel::RELIABLE);

#[derive(Serialize, Deserialize)]
pub struct EntityMoved {
    pub entity: NetworkEntity,
    pub pos: Vec3,
}
impl_message!(EntityMoved, 14, Channel::UNRELIABLE);

#[derive(Serialize, Deserialize)]
pub struct EntityRotated {
    pub entity: NetworkEntity,
    pub rot: Quat,
}
impl_message!(EntityRotated, 15, Channel::UNRELIABLE);

#[derive(Serialize, Deserialize)]
pub struct SpawnEntity {
    pub entity: NetworkEntity,
    pub pos: Vec3,
    pub rot: Quat,
}
impl_message!(SpawnEntity, 16, Channel::RELIABLE);

#[derive(Serialize, Deserialize)]
pub struct RequestChunk {
    pub pos: IVec3,
}
impl_message!(RequestChunk, 17, Channel::RELIABLE);