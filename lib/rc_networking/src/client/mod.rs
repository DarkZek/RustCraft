#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

use bevy::app::App;
use bevy::prelude::{Plugin, Resource};
use url::Url;

mod handshake;

pub struct NetworkingClientPlugin;

impl Plugin for NetworkingClientPlugin {
    fn build(&self, app: &mut App) {

        #[cfg(not(target_arch = "wasm32"))]
        {
            app.insert_resource(NetworkingClient {
                connection_status: ConnectionStatus::DISCONNECTED,
                data: native::NetworkingData::new()
            });
            app.add_plugins(native::ClientPlugin);
        }

        #[cfg(target_arch = "wasm32")]
        {
            app.insert_resource(NetworkingClient {
                connection_status: ConnectionStatus::DISCONNECTED,
                data: wasm::NetworkingData::new()
            });
            app.add_plugins(wasm::ClientPlugin);
        }

    }
}

#[derive(Resource)]
pub struct NetworkingClient {
    connection_status: ConnectionStatus,

    #[cfg(not(target_arch = "wasm32"))]
    data: native::NetworkingData,

    #[cfg(target_arch = "wasm32")]
    data: wasm::NetworkingData,
}

impl NetworkingClient {
    pub fn connect(&mut self, url: Url, user_id: u64) {
        self.data.connect(url, user_id);
    }

    pub fn disconnect(&mut self) {
        self.disconnect();
    }
}

pub enum ConnectionStatus {
    DISCONNECTED,
    CONNECTING,
    CONNECTED
}