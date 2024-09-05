use crate::bistream::BiStream;
use crate::client::native::server_connection::ServerConnection;
use crate::client::native::systems::{
    detect_shutdown_system, send_packets_system, update_system, write_packets_system,
};
use url::Url;
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;
use crate::types::{ReceivePacket, SendPacket};
use bevy::ecs::system::Resource;
use bevy::prelude::{App, debug, Plugin, Update};
use web_transport::{RecvStream};
use std::net::SocketAddr;
use std::sync::Arc;
use byteorder::{BigEndian, WriteBytesExt};
use futures::AsyncReadExt;
use quinn::{ClientConfig, Endpoint};
use rustls::RootCertStore;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::JoinHandle;
use crate::protocol::ALPN;

mod server_connection;
mod systems;

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

#[derive(Resource)]
pub struct NetworkingData {
    endpoint: Option<Endpoint>,
    client_config: ClientConfig,
    runtime: Option<Runtime>,
    connection: Option<ServerConnection>,
    pending_connection: Option<JoinHandle<ServerConnection>>,
}

impl NetworkingData {
    pub fn new() -> NetworkingData {

        let certificates = rustls_native_certs::load_native_certs().expect("could not load platform certs");

        let mut roots = RootCertStore::empty();
        roots.add_parsable_certificates(certificates);

        let mut client_config = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
            .with_protocol_versions(&[&rustls::version::TLS13]).unwrap()
            .with_root_certificates(roots)
            .with_no_client_auth();

        client_config.alpn_protocols = vec![ALPN.to_vec()];

        let client_config: quinn::crypto::rustls::QuicClientConfig = client_config.try_into().unwrap();
        let client_config = ClientConfig::new(Arc::new(
            client_config
        ));

        let runtime = Runtime::new().unwrap();

        NetworkingData {
            endpoint: None,
            client_config,
            runtime: Some(runtime),
            connection: None,
            pending_connection: None,
        }
    }

    /// Open a new endpoint connection to an address
    pub fn connect(&mut self, url: Url, user_id: u64) {
        // TODO: Move this into one async block
        let mut endpoint = self
            .runtime
            .as_mut()
            .unwrap()
            .block_on(async { Endpoint::client((std::net::Ipv6Addr::UNSPECIFIED, 0).into()).unwrap() });

        endpoint.set_default_client_config(self.client_config.clone());

        let endpoint2 = endpoint.clone();

        self.pending_connection = Some(self.runtime.as_mut().unwrap().spawn(async move {
            debug!("Starting connection to {}", url.as_str());

            let session = web_transport_quinn::connect(&endpoint2, &url).await.unwrap();

            debug!("Started connection");

            let mut session: web_transport::Session = session.into();

            let (send_err, err_recv) = unbounded_channel();

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

            let unreliable = BiStream::from_stream(unreliable.0, unreliable.1, send_err.clone());
            let reliable = BiStream::from_stream(reliable.0, reliable.1, send_err.clone());
            let chunk = BiStream::from_stream(chunk.0, chunk.1, send_err);

            debug!("Created bi streams");

            ServerConnection {
                connection: session,
                unreliable,
                reliable,
                chunk,
                err_recv,
            }
        }));

        self.endpoint = Some(endpoint);
    }
}
