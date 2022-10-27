use crate::constants::UserId;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct PlayerLeave {
    pub id: UserId,
}