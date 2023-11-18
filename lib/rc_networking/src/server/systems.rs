use crate::bistream::BiStream;
use crate::constants::UserId;
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;
use crate::protocol::clientbound::server_state::ServerState;
use crate::server::user_connection::UserConnection;
use crate::server::{NetworkingServer, USERID_COUNTER};
use crate::types::{ReceivePacket, SendPacket};
use crate::{get_channel, Channel, Protocol};
use bevy::app::AppExit;
use bevy::log::trace;
use bevy::prelude::{info, warn, EventReader, EventWriter, ResMut};
use futures::FutureExt;
use quinn::Endpoint;
use std::collections::HashMap;
use std::mem;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::unbounded_channel;

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
            if let Ok(mut new_conn) = new_connection {
                server.all_time_users += 1;

                // Send announcement
                let client = UserId(server.all_time_users);

                server.connections.insert(client, new_conn);

                connection_event.send(NetworkConnectionEvent { client });
                info!("Send connection event {:?}", server.connections.len());
            }

            // Start new connection task
            server.new_conn_task =
                Some(server.runtime.spawn(open_new_conn(server.endpoint.clone())));
        }
    }

    server.connections.retain(|userid, conn| {
        // Detect errors and disconnect from server
        return if let Err(TryRecvError::Empty) = conn.recv_err.try_recv() {
            // No events!
            true
        } else {
            // Either the writer was disconnected, or an error was given. Either way it's disconnected
            warn!("Unexpected disconnect from server {:?}", userid);
            disconnection_event.send(NetworkDisconnectionEvent { client: *userid });
            false
        };
    });
}

/// Accepts new connections then creates network channels
pub async fn open_new_conn(endpoint: Endpoint) -> UserConnection {
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
pub fn read_packets_system(
    mut server: ResMut<NetworkingServer>,
    mut recv: EventWriter<ReceivePacket>,
) {
    for (userId, client) in server.connections.iter_mut() {
        let mut recieve_from_channel = |channel: &mut BiStream| {
            while let Ok(packet) = channel.in_recv.try_recv() {
                trace!("{:?} => {:?}", userId, packet);
                recv.send(ReceivePacket(packet, *userId));
            }
        };
        recieve_from_channel(&mut client.unreliable);
        recieve_from_channel(&mut client.reliable);
        recieve_from_channel(&mut client.chunk);
    }
}

/// Take packets from ECS EventReader and add it to Writer to write to stream in other thread
pub fn write_packets_system(
    mut server: ResMut<NetworkingServer>,
    mut to_send: EventReader<SendPacket>,
) {
    to_send.iter().for_each(|v| {
        if let Some(conn) = server.connections.get(&v.1) {
            trace!("{:?} <= {:?} {:?}", v.1, get_channel(&v.0), v.0);

            match get_channel(&v.0) {
                Channel::Reliable => conn.reliable.out_send.send(v.0.clone()),
                Channel::Unreliable => conn.unreliable.out_send.send(v.0.clone()),
                Channel::Chunk => conn.chunk.out_send.send(v.0.clone()),
            }
            .unwrap();
        } else {
            trace!("Tried to send packet to disconnected client {:?}", v.1);
        }
    });
}

pub fn detect_shutdown_system(
    mut server: ResMut<NetworkingServer>,
    mut bevy_shutdown: EventReader<AppExit>,
) {
    for _ in bevy_shutdown.iter() {
        info!("Shutting down server");
        let mut connections = HashMap::new();
        mem::swap(&mut server.connections, &mut connections);

        for (_, client) in connections {
            client
                .reliable
                .out_send
                .send(Protocol::ServerState(ServerState::Disconnecting));
            client
                .connection
                .close(0_u8.into(), "Shutting Down".as_bytes())
        }
    }
}
