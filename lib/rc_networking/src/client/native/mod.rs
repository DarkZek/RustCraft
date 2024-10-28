use crate::client::native::server_connection::ServerConnection;
use crate::client::native::systems::{
    detect_shutdown_system, send_packets_system, update_system, write_packets_system,
};
use url::Url;
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;
use crate::types::{ReceivePacket, SendPacket};
use bevy::ecs::system::Resource;
use bevy::prelude::{App, debug, error, Plugin, Update};
use std::sync::Arc;
use quinn::{ClientConfig, Endpoint};
use rustls::RootCertStore;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use crate::client::handshake::{HandshakeResult, negotiate_handshake};
use crate::protocol::ALPN;
use crate::skip_verification::SkipServerVerification;

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

        let mut client_config = if !std::env::args().any(|v| v == "--unsafe-networking") {
            rustls::ClientConfig::builder_with_provider(Arc::new(
                rustls::crypto::ring::default_provider(),
            ))
                .with_protocol_versions(&[&rustls::version::TLS13]).unwrap()
                .with_root_certificates(roots)
                .with_no_client_auth()
        } else {
            rustls::crypto::ring::default_provider().install_default().unwrap();

            rustls::ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(SkipServerVerification::new())
                .with_no_client_auth()
        };

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
    pub fn connect(&mut self, url: Url, join_token: String) {
        // TODO: Move this into one async block
        let endpoint = self
            .runtime
            .as_mut()
            .unwrap()
            .block_on(async { Endpoint::client((std::net::Ipv6Addr::UNSPECIFIED, 0).into()) });

        let mut endpoint = match endpoint {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to connect to {}. Error: {:?}", url, e);
                return
            }
        };

        endpoint.set_default_client_config(self.client_config.clone());

        let endpoint2 = endpoint.clone();

        self.pending_connection = Some(self.runtime.as_mut().unwrap().spawn(async move {
            debug!("Starting connection to {}", url.as_str());

            let session = web_transport_quinn::connect(&endpoint2, &url).await.unwrap();

            debug!("Started connection");

            let mut session: web_transport::Session = session.into();

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

    pub fn disconnect(&mut self) {
        self.connection.take();
    }
}
