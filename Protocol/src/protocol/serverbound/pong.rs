use std::time::SystemTime;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Pong {
    pub code: u64
}

impl Pong {
    pub fn new() -> Pong {
        Pong {
            code: SystemTime::now().elapsed().unwrap().as_secs()
        }
    }

    pub fn from(code: u64) -> Pong {
        Pong {
            code
        }
    }
}