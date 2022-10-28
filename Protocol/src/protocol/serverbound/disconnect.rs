use crate::constants::UserId;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Disconnect {
    pub code: u32
}

impl Disconnect {
    pub fn new(code: u32) -> Disconnect {
        Disconnect {
            code
        }
    }
}