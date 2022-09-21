use naia_shared::derive_channels;
use naia_shared::{Channel, ChannelDirection, ChannelMode, ReliableSettings, TickBufferSettings};

#[derive_channels]
pub enum Channels {
    PlayerCommand,
    StatusUpdate,
    ChunkUpdates,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
    Channel {
        index: Channels::PlayerCommand,
        direction: ChannelDirection::ClientToServer,
        mode: ChannelMode::UnorderedReliable(ReliableSettings {
            rtt_resend_factor: 1.0,
        }),
    },
    Channel {
        index: Channels::StatusUpdate,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::OrderedReliable(ReliableSettings {
            rtt_resend_factor: 1.0,
        }),
    },
    Channel {
        index: Channels::ChunkUpdates,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::UnorderedReliable(ReliableSettings {
            rtt_resend_factor: 1.0,
        }),
    },
];
