use crate::constants::UserId;
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;

use crate::server::systems::{
    open_new_conn, read_packets_system, update_system, write_packets_system,
};
use crate::server::user_connection::UserConnection;
use crate::types::{ReceivePacket, SendPacket};
use crate::*;
use bevy::prelude::*;
use quinn::{Endpoint, ServerConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

mod systems;
mod user_connection;

pub struct QuinnServerPlugin;

impl Plugin for QuinnServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReceivePacket>()
            .add_event::<SendPacket>()
            .add_event::<NetworkConnectionEvent>()
            .add_event::<NetworkDisconnectionEvent>()
            .add_systems(
                Update,
                (update_system, read_packets_system, write_packets_system),
            );

        app.init_resource::<NetworkingServerConfig>();

        // Read config
        let config = app.world.get_resource::<NetworkingServerConfig>().unwrap();

        app.insert_resource(NetworkingServer::from(config));
    }
}

#[derive(Default, Resource, Debug)]
pub struct NetworkingServerConfig {
    pub cert: Option<Vec<u8>>,
    pub address: Option<SocketAddr>,
}

#[derive(Resource)]
pub struct NetworkingServer {
    _cert: Vec<u8>,
    endpoint: Endpoint,
    runtime: Runtime,
    new_conn_task: Option<JoinHandle<UserConnection>>,
    connections: HashMap<UserId, UserConnection>,
    all_time_users: u64,
}

impl Default for NetworkingServer {
    fn default() -> Self {
        NetworkingServer::from(&NetworkingServerConfig { ..default() })
    }
}

impl From<&NetworkingServerConfig> for NetworkingServer {
    /// Create Server from config
    fn from(value: &NetworkingServerConfig) -> Self {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();

        let bind_addr = if let Some(val) = value.address {
            val
        } else {
            ([127, 0, 0, 1], 25568).into()
        };

        let cert_der = if let Some(val) = &value.cert {
            val.clone()
        } else {
            cert.serialize_der().unwrap()
        };
        let priv_key = cert.serialize_private_key_der();
        let priv_key = rustls::PrivateKey(priv_key);
        let cert_chain = vec![rustls::Certificate(cert_der.clone())];

        let mut config = ServerConfig::with_single_cert(cert_chain, priv_key).unwrap();
        let transport_config = Arc::get_mut(&mut config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());
        transport_config.keep_alive_interval(Some(Duration::from_secs(5)));

        // Runtime to run Quinn in
        let runtime = Runtime::new().unwrap();

        let endpoint = runtime.block_on(async { Endpoint::server(config, bind_addr).unwrap() });

        // Start listening for new connections
        let new_conn_task = runtime.spawn(open_new_conn(endpoint.clone()));

        info!("Bound listener to {:?}", bind_addr);

        NetworkingServer {
            _cert: cert_der,
            endpoint,
            runtime,
            new_conn_task: Some(new_conn_task),
            connections: HashMap::new(),
            all_time_users: 0,
        }
    }
}
