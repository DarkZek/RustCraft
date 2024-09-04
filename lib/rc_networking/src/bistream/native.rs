use crate::Protocol;
use bevy::prelude::trace;

use web_transport::{Error, RecvStream, SendStream};
use std::mem::size_of;
use bevy::log::debug;
use futures::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::{SendError, TryRecvError};
use tokio::task::JoinHandle;
use crate::bistream::StreamError;

pub struct BiStream {
    out_handle: JoinHandle<SendStream>,
    out_send: UnboundedSender<Protocol>,
    in_handle: JoinHandle<RecvStream>,
    in_recv: UnboundedReceiver<Protocol>,
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
                    .write(bincode::serialize(&(packet_data.len() as u32)).unwrap().as_slice())
                    .await
                {
                    trace!("Network write stream failed: {:?}", e);
                    // Exiting
                    err2.send(StreamError::Error).unwrap();
                    return send;
                }

                if let Err(e) = send.write(&packet_data).await {
                    trace!("Network write stream failed: {:?}", e);
                    // Exiting
                    err2.send(StreamError::Error).unwrap();
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

            debug!("[Task Pool] [Reader] Started");

            loop {
                let len_data = match read_exact(&mut recv, size_of::<u32>()).await {
                    Ok(v) => v,
                    Err(e) => {
                        err.send(StreamError::Error).unwrap();
                        trace!("Network stream exited: {:?}", e);
                        return recv;
                    }
                };

                debug!("[Task Pool] [Reader] Received data");

                let len = bincode::deserialize::<u32>(&len_data).unwrap();

                let mut chunk_data = match read_exact(&mut recv, len as usize).await {
                    Ok(v) => v,
                    Err(e) => {
                        err.send(StreamError::Error).unwrap();
                        trace!("Network stream exited: {:?}", e);
                        return recv;
                    }
                };

                assert_eq!(chunk_data.len(), len as usize, "Chunk data was not equal for packet with length. Read: {} Expected: {}", chunk_data.len(), len);

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

    pub fn try_recv(&mut self) -> Result<Protocol, TryRecvError> {
        self.in_recv.try_recv()
    }

    pub fn send(&self, message: Protocol) -> Result<(), SendError<Protocol>> {
        self.out_send.send(message)
    }
}

async fn read_exact(recv: &mut RecvStream, len: usize) -> Result<Vec<u8>, Error> {
    // TODO: Remove copying here
    let mut chunk_data = Vec::new();
    while chunk_data.len() < len {
        let remaining_len = len - chunk_data.len();

        debug!("[Task Pool] [Reader] Remaining length {}", remaining_len);

        let mut data = recv.read(remaining_len).await?.unwrap().to_vec();

        chunk_data.append(&mut data);
    }

    Ok(chunk_data)
}