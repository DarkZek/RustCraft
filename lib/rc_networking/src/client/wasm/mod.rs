use std::mem::uninitialized;
use std::net::SocketAddr;
use bevy::app::Update;
use bevy::log::{debug, info};
use bevy::prelude::{App, Plugin, Resource, warn};
use bevy::tasks::{block_on, TaskPool};
use byteorder::{BigEndian, WriteBytesExt};
use tokio::runtime::Builder;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use url::Url;
use web_transport::wasm::{RecvStream, Session};
use crate::bistream::BiStream;
use crate::client::wasm::server_connection::ServerConnection;
use crate::client::wasm::systems::{detect_shutdown_system, send_packets_system, update_system, write_packets_system};
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;
use crate::protocol::Protocol;
use crate::types::{ReceivePacket, SendPacket};

mod systems;
mod server_connection;

pub struct QuinnClientPlugin;

impl Plugin for QuinnClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReceivePacket>()
            .add_event::<SendPacket>()
            .add_event::<NetworkConnectionEvent>()
            .add_event::<NetworkDisconnectionEvent>()
            .insert_non_send_resource(NetworkingClient::new())
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
    connection: Option<ServerConnection>,
    pending_connections_recv: Option<UnboundedReceiver<Option<ServerConnection>>>
}

impl NetworkingClient {
    pub fn new() -> NetworkingClient {
        NetworkingClient {
            connection: None,
            pending_connections_recv: None,
        }
    }

    pub fn connect(&mut self, url: Url, user_id: u64) {

        if self.pending_connections_recv.is_some() {
            warn!("Tried to connect while connection occurring");
            return;
        }

        let session = Session::connect(url);

        let (pending_connections_send, pending_connections_recv): (UnboundedSender<Option<ServerConnection>>, UnboundedReceiver<Option<ServerConnection>>) =
            unbounded_channel();

        self.pending_connections_recv = Some(pending_connections_recv);

        wasm_bindgen_futures::spawn_local(async move {

            let mut session = match session.await {
                Ok(v) => v,
                Err(e) => panic!("Server connection failed {:?}", e)
            };

            let mut unreliable = session.accept_bi().await.unwrap();
            let mut reliable = session.accept_bi().await.unwrap();
            let mut chunk = session.accept_bi().await.unwrap();

            debug!("Accepted bi streams");

            // Channel must send data to be created, so verify data sent and remove from reader
            async fn verify_stream(stream: &mut RecvStream, expected: &str) {
                let bytes = stream.read(5).await.unwrap().unwrap();
                let contents = String::from_utf8(bytes.to_vec()).unwrap();
                if contents != expected {
                    panic!(
                        "Invalid client attempted connection. Contents: {} [{:?}] Expected: {} [{:?}]",
                        contents,
                        contents.as_bytes(),
                        expected,
                        expected.as_bytes(),
                    );
                }
            }

            verify_stream(&mut unreliable.1, "Test1").await;
            verify_stream(&mut reliable.1, "Test2").await;
            verify_stream(&mut chunk.1, "Test3").await;

            debug!("Verified streams");

            let mut data = vec![];
            data.write_u64::<BigEndian>(user_id).unwrap();
            reliable.0.write(&data).await.unwrap();

            debug!("Sent UserId");

            let (send_err, err_recv) = unbounded_channel();

            let unreliable = BiStream::from_stream(unreliable.0, unreliable.1, send_err.clone());
            let reliable = BiStream::from_stream(reliable.0, reliable.1, send_err.clone());
            let chunk = BiStream::from_stream(chunk.0, chunk.1, send_err);

            debug!("Created bi streams");

            pending_connections_send.send(
                Some(
                    ServerConnection {
                        connection: session,
                        unreliable,
                        reliable,
                        chunk,
                        err_recv
                    }
                )
            ).unwrap();

            debug!("Sent successful connection");
        });
    }
}