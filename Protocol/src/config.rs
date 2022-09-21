use crate::channels::{Channels, CHANNEL_CONFIG};
use naia_shared::{
    ChannelConfig, DefaultChannels, LinkConditionerConfig, SharedConfig, SocketConfig,
};
use std::time::Duration;

pub fn network_config() -> SharedConfig<Channels> {
    // Set tick rate to ~60 FPS
    let tick_interval = Some(Duration::from_millis(20));

    //  let link_condition = None;
    let link_condition = Some(LinkConditionerConfig::good_condition());
    // let link_condition = Some(LinkConditionerConfig {
    //     incoming_latency: 500,
    //     incoming_jitter: 1,
    //     incoming_loss: 0.0,
    // });
    SharedConfig::new(
        SocketConfig::new(link_condition, None),
        CHANNEL_CONFIG,
        tick_interval,
        None,
    )
}
