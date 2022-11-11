use crate::constants::UserId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct PlayerJoin {
    pub username: [char; 32],
    pub entity: UserId,
}