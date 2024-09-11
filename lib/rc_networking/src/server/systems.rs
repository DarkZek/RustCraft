use std::mem;
use bevy::prelude::error;
use crate::bistream::{BiStream, recv_protocol, send_protocol};
use rc_shared::constants::UserId;
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;
use crate::server::user_connection::UserConnection;
use crate::server::NetworkingServer;
use crate::types::{ReceivePacket, SendPacket};
use crate::{get_channel, Channel};
use bevy::log::{trace, warn};
use bevy::prelude::{debug, EventReader, EventWriter, ResMut};
use futures::FutureExt;
use quinn::Endpoint;
use std::borrow::Borrow;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::unbounded_channel;
use web_transport::Session;
use crate::protocol::Protocol;
use crate::server::authorization::check_authorization;

/// Enables new connection attempts
pub fn update_system(
    mut server: ResMut<NetworkingServer>,
    mut connection_event: EventWriter<NetworkConnectionEvent>,
    mut disconnection_event: EventWriter<NetworkDisconnectionEvent>,
) {
    // If there is a currently running task to connect a new user
    if server.new_conn_task.is_some() {
        // Check if task is completed
        if let Some(new_connection) = server.new_conn_task.as_mut().unwrap().now_or_never() {
            // If the connection succeeded
            if let Ok(Some(new_conn)) = new_connection {

                // Send announcement
                let client = UserId(new_conn.user_authorization.sub);

                if server.connections.contains_key(&client) {
                    warn!("Already connected user attempted to connect again. Terminating connection");
                    new_conn.reliable.send(Protocol::Disconnect(String::from("User already connected."))).unwrap();
                    mem::drop(new_conn);
                } else {
                    connection_event.send(NetworkConnectionEvent {
                        client,
                        username: new_conn.user_authorization.username.clone()
                    });
                    server.connections.insert(client, new_conn);
                }
            }

            // Start new connection task
            server.new_conn_task =
                Some(server.runtime.spawn(open_new_conn(server.endpoint.clone())));
        }
    }

    server.connections.retain(|userid, conn| {
        let recv = conn.recv_err.try_recv();

        // Detect errors and disconnect from connection
        return if let Err(TryRecvError::Empty) = &recv {
            // No events!
            true
        } else {
            // Either the writer was disconnected, or an error was given. Either way it's disconnected
            warn!("Unexpected disconnect from user {:?}. {:?}", userid, recv);
            disconnection_event.send(NetworkDisconnectionEvent { client: *userid });
            false
        };
    });
}

/// Accepts new connections then creates network channels
pub async fn open_new_conn(endpoint: Endpoint) -> Option<UserConnection> {
    let connecting = endpoint.accept().await;

    let incoming_connection = match connecting {
        Some(v) => v,
        None => {
            warn!("Client failed connection with no incoming connection");
            return None
        },
    };

    let mut connecting = match incoming_connection.accept() {
        Ok(v) => v,
        Err(e) => {
            warn!("Client failed connection, {:?}", e);
            return None
        }
    };

    let handshake = connecting
        .handshake_data()
        .await
        .unwrap()
        .downcast::<quinn::crypto::rustls::HandshakeData>()
        .unwrap();

    let alpn = handshake.protocol.expect("missing ALPN");
    let alpn = String::from_utf8_lossy(&alpn);
    let server_name = handshake.server_name.unwrap_or_default();

    debug!(
        "received QUIC handshake: ip={} alpn={} connection={}",
        connecting.remote_address(),
        alpn,
        server_name,
    );

    let connection = match connecting.await {
        Ok(v) => v,
        Err(e) => {
            warn!("Client failed connection with error {:?}", e);
            return None
        }
    };

    debug!("Accepted connection");

    let mut connection: Session = match alpn.borrow() {
        "h3" => {
            // HTTP3
            let request = web_transport_quinn::accept(connection)
                .await
                .unwrap();

            debug!("Accepted HTTP3 handshake");

            let session: web_transport_quinn::Session = request
                .ok()
                .await
                .unwrap();

            Session::from(session)
        }
        v => {
            warn!("Unsupported ALPN attempted to connect {}", v);
            return None;
        }
    };

    debug!("Negotiated HTTP3");

    let mut unreliable = connection.open_bi().await.unwrap();
    let mut reliable = connection.open_bi().await.unwrap();
    let mut chunk = connection.open_bi().await.unwrap();

    debug!("Opened Bi Streams");

    // Stream not created until written to, so to ensure order write 5 bytes
    unreliable
        .0
        .write("Test1".as_bytes().into())
        .await
        .unwrap();
    reliable
        .0
        .write("Test2".as_bytes().into())
        .await
        .unwrap();
    chunk.0.write("Test3".as_bytes().into()).await.unwrap();

    debug!("Sent validation packets");

    let authorization = recv_protocol(&mut reliable.1).await.unwrap();

    let Protocol::Authorization(token) = authorization else {
        warn!("New connection attempted to skip authorization");
        return None
    };

    trace!("Received authorization");

    let user_authorization = match check_authorization(&token) {
        Some(v) => v,
        // Terminate connection
        None => return None
    };

    send_protocol(&Protocol::AuthorizationAccepted, &mut reliable.0).await.unwrap();

    let (send_err, recv_err) = unbounded_channel();

    let unreliable = BiStream::from_stream(unreliable.0, unreliable.1, send_err.clone());
    let reliable = BiStream::from_stream(reliable.0, reliable.1, send_err.clone());
    let chunk = BiStream::from_stream(chunk.0, chunk.1, send_err);

    debug!("Successfully create BiStreams");

    Some(UserConnection {
        connection,
        unreliable,
        reliable,
        chunk,
        recv_err,
        user_authorization,
    })
}

/// Detect shutdowns and close networking client
pub fn read_packets_system(
    mut server: ResMut<NetworkingServer>,
    mut recv: EventWriter<ReceivePacket>,
) {
    for (user_id, client) in server.connections.iter_mut() {
        let mut recieve_from_channel = |channel: &mut BiStream| {
            while let Ok(packet) = channel.try_recv() {
                trace!("{:?} => {:?}", user_id, packet);
                recv.send(ReceivePacket(packet, *user_id));
            }
        };
        recieve_from_channel(&mut client.unreliable);
        recieve_from_channel(&mut client.reliable);
        recieve_from_channel(&mut client.chunk);
    }
}

/// Take packets from ECS EventReader and add it to Writer to write to stream in other thread
pub fn write_packets_system(
    server: ResMut<NetworkingServer>,
    mut to_send: EventReader<SendPacket>,
) {
    to_send.read().for_each(|v| {
        if let Some(conn) = server.connections.get(&v.1) {
            trace!("{:?} <= {:?} {:?}", v.1, get_channel(&v.0), v.0);

            let result = match get_channel(&v.0) {
                Channel::Reliable => conn.reliable.send(v.0.clone()),
                Channel::Unreliable => conn.unreliable.send(v.0.clone()),
                Channel::Chunk => conn.chunk.send(v.0.clone()),
            };

            if let Err(e) = result {
                error!("Failed to send packet {:?}", e);
            }
        } else {
            trace!("Tried to send packet to disconnected client {:?}", v.1);
        }
    });
}
