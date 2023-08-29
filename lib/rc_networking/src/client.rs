use crate::bistream::{BiStream, StreamError};
use crate::connection::NetworkConnectionEvent;
use crate::constants::UserId;
use crate::disconnect::NetworkDisconnectionEvent;
use crate::types::{ReceivePacket, SendPacket};
use crate::*;
use bevy::app::AppExit;
use bevy::prelude::KeyCode::Apps;
use bevy::prelude::*;
use quinn::{ClientConfig, Connection, Endpoint, RecvStream, SendStream};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

pub struct QuinnClientPlugin;

impl Plugin for QuinnClientPlugin {
    fn build(&self, app: &mut App) {
        // TODO
        app.add_event::<ReceivePacket>()
            .add_event::<SendPacket>()
            .add_event::<NetworkConnectionEvent>()
            .add_event::<NetworkDisconnectionEvent>()
            .insert_resource(NetworkingClient::new())
            .add_systems(
                Update,
                (
                    send_packets_system,
                    write_packets_system,
                    detect_shutdown_system,
                ),
            );
    }
}

/// Take packets from ECS EventReader and add it to Writer to write to stream in other thread
fn write_packets_system(
    mut client: ResMut<NetworkingClient>,
    mut to_send: EventReader<SendPacket>,
) {
    if to_send.len() == 0 {
        return;
    }
    if let Some(conn) = &client.connection {
        for packet in to_send.iter() {
            match get_channel(&packet.0) {
                Channel::Reliable => conn.reliable.out_send.send(packet.0.clone()),
                Channel::Unreliable => conn.unreliable.out_send.send(packet.0.clone()),
                Channel::Chunk => conn.chunk.out_send.send(packet.0.clone()),
            }
            .unwrap();
        }
    } else {
        warn!(
            "Tried to send packet when disconnected {:?}",
            to_send.iter().collect::<Vec<&SendPacket>>()
        );
    }
}

/// Detect shutdowns and close networking client
fn detect_shutdown_system(
    mut client: ResMut<NetworkingClient>,
    mut bevy_shutdown: EventReader<AppExit>,
) {
    for _ in bevy_shutdown.iter() {
        info!("Shutting down server");
        if let Some(connection) = client.connection.take() {
            connection
                .connection
                .close(0_u8.into(), "Closed".as_bytes());
        }
        if let Some(endpoint) = client.endpoint.take() {
            endpoint.close(0_u8.into(), "Closed".as_bytes());
        }
        if let Some(runtime) = client.runtime.take() {
            runtime.shutdown_background();
        }
    }
}

/// Take packets from Receivers and add it to ECS EventWriter
fn send_packets_system(mut client: ResMut<NetworkingClient>, mut recv: EventWriter<ReceivePacket>) {
    if let Some(conn) = &mut client.connection {
        let mut recieve_from_channel = |channel: &mut BiStream| {
            while let Ok(packet) = channel.in_recv.try_recv() {
                recv.send(ReceivePacket(packet, UserId(0)));
            }
        };
        recieve_from_channel(&mut conn.unreliable);
        recieve_from_channel(&mut conn.reliable);
        recieve_from_channel(&mut conn.chunk);
    }
}

struct ServerConnection {
    connection: Connection,
    unreliable: BiStream,
    reliable: BiStream,
    chunk: BiStream,
    err_recv: UnboundedReceiver<StreamError>,
}

#[derive(Resource)]
pub struct NetworkingClient {
    endpoint: Option<Endpoint>,
    client_config: ClientConfig,
    runtime: Option<Runtime>,
    connection: Option<ServerConnection>,
}

impl NetworkingClient {
    pub fn new() -> NetworkingClient {
        let client_config = ClientConfig::new(Arc::new(
            rustls::ClientConfig::builder()
                .with_safe_defaults()
                .with_custom_certificate_verifier(SkipServerVerification::new())
                .with_no_client_auth(),
        ));

        let runtime = Runtime::new().unwrap();

        NetworkingClient {
            endpoint: None,
            client_config,
            runtime: Some(runtime),
            connection: None,
        }
    }

    /// Open a new endpoint connection to an address
    pub fn connect(&mut self, addr: SocketAddr) {
        let mut endpoint = self
            .runtime
            .as_mut()
            .unwrap()
            .block_on(async { Endpoint::client("127.0.0.1:0".parse().unwrap()).unwrap() });

        endpoint.set_default_client_config(self.client_config.clone());

        let connection = self.runtime.as_mut().unwrap().block_on(async {
            let connection = endpoint.connect(addr, "localhost").unwrap().await.unwrap();

            let mut unreliable = connection.accept_bi().await.unwrap();
            let mut reliable = connection.accept_bi().await.unwrap();
            let mut chunk = connection.accept_bi().await.unwrap();

            // Channel must send data to be created, so verify data sent and remove from reader
            async fn verify_stream(stream: &mut RecvStream, expected: &str) {
                let mut data = vec![0; 5];
                stream.read_exact(&mut data).await.unwrap();
                let contents = String::from_utf8(data).unwrap();
                if contents != expected {
                    panic!(
                        "Invalid client attempted connection. Contents: {}",
                        contents
                    );
                }
            }

            verify_stream(&mut unreliable.1, "Test1").await;
            verify_stream(&mut reliable.1, "Test2").await;
            verify_stream(&mut chunk.1, "Test3").await;

            let (send_err, err_recv) = unbounded_channel();

            let unreliable = BiStream::from_stream(unreliable.0, unreliable.1, send_err.clone());
            let reliable = BiStream::from_stream(reliable.0, reliable.1, send_err.clone());
            let chunk = BiStream::from_stream(chunk.0, chunk.1, send_err);

            ServerConnection {
                connection,
                unreliable,
                reliable,
                chunk,
                err_recv,
            }
        });

        self.endpoint = Some(endpoint);
        self.connection = Some(connection);
    }
}

struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        warn!("Ignoring server tls certificate checking");
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
