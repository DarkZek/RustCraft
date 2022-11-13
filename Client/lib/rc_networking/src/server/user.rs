use crossbeam::channel::{Receiver, Sender};
use tokio::task::JoinHandle;
use rc_protocol::constants::UserId;
use rc_protocol::types::{ReceivePacket, SendPacket};

pub struct NetworkUser {
    pub id: UserId,

    pub read_packets: Receiver<ReceivePacket>,
    pub write_packets: Sender<SendPacket>,

    pub read_packet_handle: JoinHandle<()>,
    pub write_packet_handle: JoinHandle<()>,
}