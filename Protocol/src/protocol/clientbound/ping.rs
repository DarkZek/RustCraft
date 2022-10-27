use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Ping {
    pub code: u64
}

impl Ping {
    pub fn new() -> Ping {
        Ping {
            code: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        }
    }

    pub fn from(code: u64) -> Ping {
        Ping {
            code
        }
    }
}