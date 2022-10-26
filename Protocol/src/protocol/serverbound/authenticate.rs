use crate::constants::UserId;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserAuthenticate {
    pub username: String,
    pub entity: UserId,
}