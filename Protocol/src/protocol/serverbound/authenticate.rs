use crate::constants::UserId;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserAuthenticate {
    pub username: [char; 32],
    pub entity: UserId,
}