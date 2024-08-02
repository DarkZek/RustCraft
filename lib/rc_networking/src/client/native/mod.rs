use crate::bistream::BiStream;
use crate::client::native::server_connection::ServerConnection;
use crate::client::native::skip_verification::SkipServerVerification;
use crate::client::native::systems::{
    detect_shutdown_system, send_packets_system, update_system, write_packets_system,
};
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;
use crate::types::{ReceivePacket, SendPacket};
use bevy::ecs::system::Resource;
use bevy::prelude::{App, Plugin, Update};
use quinn::{ClientConfig, Endpoint, RecvStream};
use std::net::SocketAddr;
use std::sync::Arc;
use byteorder::{BigEndian, WriteBytesExt};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::JoinHandle;

mod server_connection;
mod skip_verification;
mod systems;

pub struct QuinnClientPlugin;

impl Plugin for QuinnClientPlugin {
    fn build(&self, app: &mut App) {
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
                    update_system,
                ),
            );
    }
}

#[derive(Resource)]
pub struct NetworkingClient {
    endpoint: Option<Endpoint>,
    client_config: ClientConfig,
    runtime: Option<Runtime>,
    connection: Option<ServerConnection>,
    pending_connection: Option<JoinHandle<ServerConnection>>,
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
            pending_connection: None,
        }
    }

    /// Open a new endpoint connection to an address
    pub fn connect(&mut self, addr: SocketAddr, user_id: u64) {
        let mut endpoint = self
            .runtime
            .as_mut()
            .unwrap()
            .block_on(async { Endpoint::client("127.0.0.1:0".parse().unwrap()).unwrap() });

        endpoint.set_default_client_config(self.client_config.clone());

        let endpoint2 = endpoint.clone();

        self.pending_connection = Some(self.runtime.as_mut().unwrap().spawn(async move {
            let connection = endpoint2.connect(addr, "localhost").unwrap().await.unwrap();

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

            let mut data = vec![];
            data.write_u64::<BigEndian>(user_id).unwrap();
            reliable.0.write_all(&*data).await.unwrap();

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
        }));

        self.endpoint = Some(endpoint);
    }
}
