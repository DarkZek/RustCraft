use crate::services::networking_service::NetworkingService;
use rc_network::messaging::NetworkingMessage;
use rc_network::protocol::packet::serverbound::ServerBoundPacketData;
use specs::{Component, DenseVecStorage, Entities, Join, System, Write, WriteStorage};

#[derive(Debug)]
pub struct SendNetworkPacket(pub ServerBoundPacketData);

impl Component for SendNetworkPacket {
    type Storage = DenseVecStorage<Self>;
}

pub struct NetworkingSendSystem;

impl<'a> System<'a> for NetworkingSendSystem {
    type SystemData = (
        WriteStorage<'a, SendNetworkPacket>,
        Write<'a, NetworkingService>,
        Entities<'a>,
    );

    fn run(&mut self, (mut send_packets, mut service, mut entities): Self::SystemData) {
        if send_packets.is_empty() {
            return;
        }

        let mut packets = Vec::new();

        for (entity, packet) in (&entities, send_packets.drain()).join() {
            entities.delete(entity);
            packets.push(packet.0);
        }

        service
            .networking
            .send_message(NetworkingMessage::PacketQueue(packets));
    }
}
