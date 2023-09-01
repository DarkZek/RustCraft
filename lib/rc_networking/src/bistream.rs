use crate::protocol::clientbound::update_loading::UpdateLoading;
use crate::Protocol;
use bevy::prelude::{info, trace};
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
                if let Err(e) = send
                    .write_all(&bincode::serialize(&(packet_data.len() as u32)).unwrap())
                    .await
                {
                    trace!("Network write stream failed: {:?}", e);
                    // Exiting
                    err2.send(StreamError::Error);
                    return send;
                }

                if let Err(e) = send.write_all(&packet_data).await {
                    trace!("Network write stream failed: {:?}", e);
                    // Exiting
                    err2.send(StreamError::Error);
                    return send;
                }

                trace!("<= {:?}", packet);
            }

            trace!("Network stream exited");

            send
        });

        let (in_send, in_recv): (UnboundedSender<Protocol>, UnboundedReceiver<Protocol>) =
            unbounded_channel();
        let in_handle = tokio::spawn(async move {
            loop {
                let mut len_data = vec![0; size_of::<u32>()];
                if let Err(e) = recv.read_exact(&mut len_data).await {
                    err.send(StreamError::Error);
                    trace!("Network stream exited: {:?}", e);
                    return recv;
                }

                let len = bincode::deserialize::<u32>(&len_data).unwrap();

                let mut chunk_data = vec![0; len as usize];

                if let Err(e) = recv.read_exact(&mut chunk_data).await {
                    err.send(StreamError::Error);
                    trace!("Network read stream failed: {:?}", e);
                    return recv;
                }

                let data = bincode::deserialize::<Protocol>(&chunk_data).unwrap();
                trace!("=> {:?}", data);
                in_send.send(data).unwrap();
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
