use rc_shared::constants::UserId;
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;

use crate::server::systems::{
    open_new_conn, read_packets_system, update_system, write_packets_system,
};
use crate::server::user_connection::UserConnection;
use crate::types::{ReceivePacket, SendPacket};
use crate::*;
use bevy::prelude::*;
use quinn::{Endpoint, IdleTimeout, ServerConfig};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use rcgen::KeyPair;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use url::{Url};
use crate::protocol::ALPN;

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
        let config = app.world().get_resource::<NetworkingServerConfig>().unwrap();

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
    endpoint: Endpoint,
    runtime: Runtime,
    new_conn_task: Option<JoinHandle<Option<UserConnection>>>,
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
        let bind_addr = if let Some(val) = value.address.clone() {
            val
        } else {
            "[::]:25568".parse().unwrap()
        };

        if let Some(_) = &value.cert {
            panic!("Passing in custom certificate unsupported");
        }

        let (certificates, private_key) = get_certificates(
            "cert.pem",
            "key.pem"
        );

        let mut config = rustls::ServerConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
            .with_protocol_versions(&[&rustls::version::TLS13]).unwrap()
            .with_no_client_auth()
            .with_single_cert(
                certificates,
                private_key
            )
            .unwrap();

        config.max_early_data_size = u32::MAX;
        config.alpn_protocols = vec![ALPN.to_vec()];

        let config: quinn::crypto::rustls::QuicServerConfig = config.try_into().unwrap();
        let mut config = ServerConfig::with_crypto(Arc::new(config));

        let transport_config = Arc::get_mut(&mut config.transport).unwrap();
        // transport_config.max_concurrent_uni_streams(0_u8.into());
        transport_config.keep_alive_interval(Some(Duration::from_millis(250)));
        // transport_config.max_idle_timeout(Some(IdleTimeout::try_from(Duration::from_millis(2000)).unwrap()));

        // Runtime to run Quinn in
        let runtime = Runtime::new().unwrap();

        let endpoint = runtime.block_on(async { Endpoint::server(config, bind_addr).unwrap() });

        // Start listening for new connections
        let new_conn_task = runtime.spawn(open_new_conn(endpoint.clone()));

        info!("Bound listener to {:?}", bind_addr);

        NetworkingServer {
            endpoint,
            runtime,
            new_conn_task: Some(new_conn_task),
            connections: HashMap::new(),
            all_time_users: 0,
        }
    }
}

fn get_certificates(cert_file: &str, private_key_file: &str) -> (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>) {

    let Ok(mut cert) = File::open(cert_file) else {
        panic!("Certificate {} file does not exist", cert_file)
    };
    let Ok(mut private_key) = File::open(private_key_file) else {
        panic!("Private key {} file does not exist", private_key_file)
    };

    let certs = rustls_pemfile::certs(&mut BufReader::new(&mut cert))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let private_key =
        rustls_pemfile::private_key(&mut BufReader::new(&mut private_key))
            .unwrap()
            .unwrap();

    (certs, private_key)
}