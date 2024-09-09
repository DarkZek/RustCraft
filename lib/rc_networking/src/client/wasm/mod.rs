use std::future::Future;
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
use web_transport::Error;
use web_transport::wasm::{RecvStream, Session};
use rc_shared::constants::UserId;
use crate::bistream::BiStream;
use crate::client::handshake::{HandshakeResult, negotiate_handshake};
use crate::client::wasm::server_connection::ServerConnection;
use crate::client::wasm::systems::{detect_shutdown_system, send_packets_system, update_system, write_packets_system};
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;
use crate::protocol::Protocol;
use crate::types::{ReceivePacket, SendPacket};

mod systems;
mod server_connection;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReceivePacket>()
            .add_event::<SendPacket>()
            .add_event::<NetworkConnectionEvent>()
            .add_event::<NetworkDisconnectionEvent>()
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

unsafe impl Send for NetworkingData {}
unsafe impl Sync for NetworkingData {}

#[derive(Resource)]
pub struct NetworkingData {
    connection: Option<ServerConnection>,
    pending_connections_recv: Option<UnboundedReceiver<Option<ServerConnection>>>
}

impl NetworkingData {
    pub fn new() -> NetworkingData {
        NetworkingData {
            connection: None,
            pending_connections_recv: None,
        }
    }

    pub fn connect(&mut self, url: Url, join_token: String) {

        if self.pending_connections_recv.is_some() {
            warn!("Tried to connect while connection occurring");
            return;
        }

        let session = Session::connect(url);

        let (pending_connections_send, pending_connections_recv): (UnboundedSender<Option<ServerConnection>>, UnboundedReceiver<Option<ServerConnection>>) =
            unbounded_channel();

        self.pending_connections_recv = Some(pending_connections_recv);

        wasm_bindgen_futures::spawn_local(async move {

            let session = match session.await {
                Ok(v) => v,
                Err(e) => panic!("Server connection failed {:?}", e)
            };

            let mut session = web_transport::Session::from(session);

            let handshake_result = match negotiate_handshake(&mut session, join_token).await {
                Ok(v) => v,
                Err(e) => panic!("Server connection failed {:?}", e)
            };

            let HandshakeResult {
                unreliable,
                reliable,
                chunk,
                err_recv
            } = handshake_result;

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

    pub fn disconnect(&mut self) {
        self.connection.take();
    }
}