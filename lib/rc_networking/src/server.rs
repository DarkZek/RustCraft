use crate::bistream::{BiStream, StreamError};
use crate::connection::NetworkConnectionEvent;
use crate::constants::UserId;
use crate::disconnect::NetworkDisconnectionEvent;
use crate::protocol::clientbound::update_loading::UpdateLoading;
use crate::types::{ReceivePacket, SendPacket};
use crate::*;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;
use futures::{AsyncWriteExt, FutureExt};
use quinn::{Connection, Endpoint, RecvStream, SendStream, ServerConfig};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::task::JoinHandle;

const USERID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct QuinnServerPlugin;

impl Plugin for QuinnServerPlugin {
    fn build(&self, app: &mut App) {
        // TODO
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

/// Enables new connection attempts
fn update_system(
    mut server: ResMut<NetworkingServer>,
    mut connection_event: EventWriter<NetworkConnectionEvent>,
) {
    // If there is a currently running task to connect a new user
    if server.new_conn_task.is_some() {
        // Check if task is completed
        if let Some(new_connection) = server.new_conn_task.as_mut().unwrap().now_or_never() {
            // If the connection succeeded
            if let Ok(mut new_conn) = new_connection {
                // Send announcement
                let client = UserId(USERID_COUNTER.fetch_add(1, Ordering::Acquire));

                server.connections.insert(client, new_conn);

                connection_event.send(NetworkConnectionEvent { client });
                info!("Send connection event");
            }

            // Start new connection task
            server.new_conn_task =
                Some(server.runtime.spawn(open_new_conn(server.endpoint.clone())));
        }
    }
}

struct NewConnection {
    pub connection: Connection,
    pub unreliable: (SendStream, RecvStream),
    pub reliable: (SendStream, RecvStream),
    pub chunk: (SendStream, RecvStream),
}

struct UserConnection {
    pub connection: Connection,
    pub unreliable: BiStream,
    pub reliable: BiStream,
    pub chunk: BiStream,
    pub recv_err: UnboundedReceiver<StreamError>,
}

/// Accepts new connections then creates network channels
async fn open_new_conn(endpoint: Endpoint) -> UserConnection {
    let connecting = endpoint.accept().await;
    let connection = connecting.unwrap().await.unwrap();

    let mut unreliable = connection.open_bi().await.unwrap();
    let mut reliable = connection.open_bi().await.unwrap();
    let mut chunk = connection.open_bi().await.unwrap();

    // Stream not created until written to, so to ensure order write 5 bytes
    unreliable
        .0
        .write_all("Test1".as_bytes().into())
        .await
        .unwrap();
    reliable
        .0
        .write_all("Test2".as_bytes().into())
        .await
        .unwrap();
    chunk.0.write_all("Test3".as_bytes().into()).await.unwrap();

    let (send_err, recv_err) = unbounded_channel();

    let unreliable = BiStream::from_stream(unreliable.0, unreliable.1, send_err.clone());
    let reliable = BiStream::from_stream(reliable.0, reliable.1, send_err.clone());
    let chunk = BiStream::from_stream(chunk.0, chunk.1, send_err);

    UserConnection {
        connection,
        unreliable,
        reliable,
        chunk,
        recv_err,
    }
}

/// Detect shutdowns and close networking client
fn read_packets_system(mut server: ResMut<NetworkingServer>, mut recv: EventWriter<ReceivePacket>) {
    for (userId, client) in server.connections.iter_mut() {
        let mut recieve_from_channel = |channel: &mut BiStream| {
            while let Ok(packet) = channel.in_recv.try_recv() {
                info!("{:?} => {:?}", userId, packet);
                recv.send(ReceivePacket(packet, *userId));
            }
        };
        recieve_from_channel(&mut client.unreliable);
        recieve_from_channel(&mut client.reliable);
        recieve_from_channel(&mut client.chunk);
    }
}

/// Take packets from ECS EventReader and add it to Writer to write to stream in other thread
fn write_packets_system(
    mut server: ResMut<NetworkingServer>,
    mut to_send: EventReader<SendPacket>,
) {
    to_send.iter().for_each(|v| {
        let conn = server.connections.get(&v.1).unwrap();
        info!("{:?} <= {:?} {:?}", v.1, get_channel(&v.0), v.0);

        match get_channel(&v.0) {
            Channel::Reliable => conn.reliable.out_send.send(v.0.clone()),
            Channel::Unreliable => conn.unreliable.out_send.send(v.0.clone()),
            Channel::Chunk => conn.chunk.out_send.send(v.0.clone()),
        }
        .unwrap();
    });
}

fn detect_shutdown_system(
    mut server: ResMut<NetworkingServer>,
    mut bevy_shutdown: EventReader<AppExit>,
) {
    for _ in bevy_shutdown.iter() {
        info!("Shutting down server");
        // TODO
    }
}

#[derive(Default, Resource, Debug)]
pub struct NetworkingServerConfig {
    pub cert: Option<Vec<u8>>,
    pub address: Option<SocketAddr>,
}

#[derive(Resource)]
struct NetworkingServer {
    cert: Vec<u8>,
    endpoint: Endpoint,
    runtime: Runtime,
    new_conn_task: Option<JoinHandle<UserConnection>>,
    connections: HashMap<UserId, UserConnection>,
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

        // Runtime to run Quinn in
        let runtime = Runtime::new().unwrap();

        let endpoint = runtime.block_on(async { Endpoint::server(config, bind_addr).unwrap() });

        // Start listening for new connections
        let new_conn_task = runtime.spawn(open_new_conn(endpoint.clone()));

        info!("Bound listener to {:?}", bind_addr);

        NetworkingServer {
            cert: cert_der,
            endpoint,
            runtime,
            new_conn_task: Some(new_conn_task),
            connections: HashMap::new(),
        }
    }
}
