use crate::protocol::clientbound::block_update::BlockUpdate;
use crate::protocol::clientbound::chat::ChatSent;
use crate::protocol::clientbound::chunk_update::PartialChunkUpdate;
use crate::protocol::clientbound::player_join::PlayerJoin;
use crate::protocol::clientbound::player_leave::PlayerLeave;
use crate::protocol::clientbound::player_moved::PlayerMoved;
use crate::protocol::clientbound::player_rotated::PlayerRotated;
use crate::protocol::serverbound::authenticate::UserAuthenticate;
use crate::protocol::serverbound::player_move::PlayerMove;
use crate::protocol::serverbound::player_rotate::PlayerRotate;
use naia_shared::Protocolize;

pub mod clientbound;
pub mod serverbound;

#[derive(Protocolize)]
pub enum Protocol {
    PlayerJoin(PlayerJoin),
    PlayerMove(PlayerMove),
    PlayerMoved(PlayerMoved),
    PlayerRotate(PlayerRotate),
    PlayerRotated(PlayerRotated),
    PlayerLeave(PlayerLeave),
    BlockUpdate(BlockUpdate),
    ChatSent(ChatSent),
    PartialChunkUpdate(PartialChunkUpdate),
    UserAuthenticate(UserAuthenticate),
}
