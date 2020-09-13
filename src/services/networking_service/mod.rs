use crate::services::networking_service::system::ReceivedNetworkPackets;
use rc_network::messaging::NetworkingMessage;
use rc_network::protocol::packet::PacketData;
use rc_network::RustcraftNetworking;
use specs::World;

pub mod system;

pub struct NetworkingService {
    networking: RustcraftNetworking,
}

impl NetworkingService {
    pub fn new(universe: &mut World) -> NetworkingService {
        let service = RustcraftNetworking::new();
        service.start();

        service.send_message(NetworkingMessage::Connect("localhost".to_string(), 25565));
        universe.insert(ReceivedNetworkPackets { packets: vec![] });

        NetworkingService {
            networking: service,
        }
    }

    pub fn connect_to_server(&mut self, ip: String, port: u32) {
        self.networking
            .send_message(NetworkingMessage::Connect(ip, port));
    }

    pub fn shutdown(&mut self) {
        self.networking.send_message(NetworkingMessage::Shutdown);
    }

    pub fn get_packets(&mut self) -> Vec<PacketData> {
        self.networking.get_packets()
    }
}

impl Default for NetworkingService {
    fn default() -> Self {
        unimplemented!()
    }
}
