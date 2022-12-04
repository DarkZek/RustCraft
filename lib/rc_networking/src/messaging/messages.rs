use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::client::Client;
use crate::server::Server;
use crate::config::Channel;
use crate::messaging::serialize::make_serializer;
use crate::impl_message;

make_serializer![
    BlockUpdate
];

#[derive(Serialize, Deserialize)]
pub struct BlockUpdate {
    pub id: u32,
    pub pos: IVec3,
}
impl_message!(BlockUpdate, 10, Channel::RELIABLE);