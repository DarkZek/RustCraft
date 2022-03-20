use crate::services::networking_service::NetworkingService;
use rc_network::protocol::packet::serverbound::ServerBoundPacketData;
use specs::{Component, DenseVecStorage, Join, System, Write, WriteStorage};

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
    );

    fn run(&mut self, (mut packets, mut service): Self::SystemData) {
        for packet in (packets).join() {
            println!("Packet: {:?}", *packet);
        }
    }
}
