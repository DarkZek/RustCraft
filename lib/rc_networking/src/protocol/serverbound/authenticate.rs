use crate::constants::UserId;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct UserAuthenticate {
    pub username: [char; 32],
    pub entity: UserId,
}