use crate::constants::UserId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PlayerJoin {
    pub username: String,
    pub entity: UserId,
}