use crate::protocol::clientbound::update_loading::UpdateLoading;
use crate::Protocol;
use bevy::prelude::info;
use futures::task::SpawnExt;
use quinn::{RecvStream, SendStream};
use std::mem::size_of;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

const MAX_CHUNK_SIZE: usize = 1024 * 32;

pub enum StreamError {
    Error,
}

pub struct BiStream {
    pub out_handle: JoinHandle<SendStream>,
    pub out_send: UnboundedSender<Protocol>,
    pub in_handle: JoinHandle<RecvStream>,
    pub in_recv: UnboundedReceiver<Protocol>,
}

impl BiStream {
    pub fn from_stream(
        mut send: SendStream,
        mut recv: RecvStream,
        err: UnboundedSender<StreamError>,
    ) -> BiStream {
        let err2 = err.clone();

        let (out_send, mut out_recv): (UnboundedSender<Protocol>, UnboundedReceiver<Protocol>) =
            unbounded_channel();

        // Spawn new runtime
        let out_handle = tokio::spawn(async move {
            while let Some(packet) = out_recv.recv().await {
                let packet_data = bincode::serialize(&packet).unwrap();
                send.write_all(&bincode::serialize(&(packet_data.len() as u32)).unwrap())
                    .await
                    .unwrap();
                send.write_all(&packet_data).await.unwrap();
                info!("<= {:?}", packet);
            }

            // Exiting
            err2.send(StreamError::Error);

            send
        });

        let (in_send, in_recv): (UnboundedSender<Protocol>, UnboundedReceiver<Protocol>) =
            unbounded_channel();
        let in_handle = tokio::spawn(async move {
            loop {
                let mut len_data = vec![0; size_of::<u32>()];
                recv.read_exact(&mut len_data).await.unwrap();

                let len = bincode::deserialize::<u32>(&len_data).unwrap();
                println!("Length {}", len);

                let mut chunk_data = vec![0; len as usize];

                if recv.read_exact(&mut chunk_data).await.is_err() {
                    err.send(StreamError::Error);
                    return recv;
                }

                let data = bincode::deserialize::<Protocol>(&chunk_data).unwrap();
                info!("=> {:?}", data);
                in_send.send(data);
            }
        });

        BiStream {
            out_handle,
            out_send,
            in_handle,
            in_recv,
        }
    }
}
