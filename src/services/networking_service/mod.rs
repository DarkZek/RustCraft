use crate::services::networking_service::recieve_system::ReceivedNetworkPackets;
use crate::services::settings_service::SettingsService;
use rc_network::messaging::NetworkingMessage;
use rc_network::protocol::packet::clientbound::ClientBoundPacketData;
use rc_network::RustcraftNetworking;
use specs::World;

pub mod recieve_system;
pub mod send_system;

pub struct NetworkingService {
    networking: RustcraftNetworking,
    username: String,
}

impl NetworkingService {
    pub fn new(universe: &mut World, settings: &SettingsService) -> NetworkingService {
        let service = RustcraftNetworking::new();
        service.start();

        universe.insert(ReceivedNetworkPackets { packets: vec![] });

        NetworkingService {
            networking: service,
            username: settings.config.username.clone(),
        }
    }

    pub fn connect_to_server(&mut self, ip: String, port: u32) {
        self.networking
            .send_message(NetworkingMessage::Connect(ip, port, self.username.clone()));
    }

    pub fn shutdown(&self) {
        self.networking.send_message(NetworkingMessage::Shutdown);
    }

    pub fn get_packets(&mut self) -> Vec<ClientBoundPacketData> {
        self.networking.get_packets()
    }
}

impl Default for NetworkingService {
    fn default() -> Self {
        unimplemented!()
    }
}
