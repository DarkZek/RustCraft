use std::time::Duration;
use renet::*;

pub const PROTOCOL_ID: u64 = 4302467916224429941;

// current private key is SHA256 hash of format!("{}{}", PROTOCOL_ID, "RustCraft");
pub const PRIVATE_KEY: [u8; 32] = [
    0x2e, 0x7c, 0x89, 0x9c, 0xf6, 0x46, 0x8d, 0x19,
    0x4b, 0x38, 0x14, 0xfd, 0xea, 0xa8, 0x7a, 0xce,
    0xf2, 0xc7, 0x2d, 0x99, 0x2b, 0x1b, 0xe2, 0x5d,
    0x29, 0x2d, 0xd3, 0x26, 0x52, 0x71, 0x8a, 0x1b
];

macro_rules! make_channels {
    ($($n:tt),*) => {
        #[derive(Copy, Clone)]
        pub enum Channel {
            $($n),*
        }
        impl From<Channel> for u8 {
            fn from(value: Channel) -> Self {
                make_channels!(@matcher 0u8, value, {}, $($n)*)
            }
        }
    };
    (@matcher $_idx: expr, $value:expr, {$($arms:tt)*}, $(,)*) => {
        match $value {
            $($arms)*
        }
    };
    (@matcher $idx: expr, $value:expr, {$($arms:tt)*}, $head:tt $($tail:tt)*) => {
        make_channels!(@matcher $idx + 1u8, $value, {$($arms)* Channel::$head => { $idx }}, $($tail)*);
    };
}

make_channels!(Reliable, Unreliable, Block);

pub fn get_renet_connection_config() -> RenetConnectionConfig {
    let channels_config = vec![
        ChannelConfig::Reliable(
            ReliableChannelConfig {
                channel_id: Channel::Reliable.into(),
                ..Default::default()
            }
        ),
        ChannelConfig::Unreliable(
            UnreliableChannelConfig {
                channel_id: Channel::Unreliable.into(),
                ..Default::default()
            }
        ),
        ChannelConfig::Block(BlockChannelConfig {
            channel_id: Channel::Block.into(),
            slice_size: 1024,
            resend_time: Duration::from_millis(300),
            sent_packet_buffer_size: 256,
            packet_budget: 8 * 1024,
            max_message_size: 256 * 1024,
            message_send_queue_size: 1024,
        }),
    ];

    let config = RenetConnectionConfig {
        max_packet_size: 16 * 1024,
        sent_packets_buffer_size: 256,
        received_packets_buffer_size: 256,
        reassembly_buffer_size: 256,
        rtt_smoothing_factor: 0.005,
        packet_loss_smoothing_factor: 0.1,
        bandwidth_smoothing_factor: 0.1,
        heartbeat_time: Duration::from_millis(100),
        send_channels_config: channels_config.clone(),
        receive_channels_config: channels_config,
    };

    config
}