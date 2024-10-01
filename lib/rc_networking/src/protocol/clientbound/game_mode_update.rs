use serde::{Serialize, Deserialize};
use rc_shared::game_mode::PlayerGameMode;

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct GameModeUpdate {
    pub game_mode: PlayerGameMode
}
