use crate::Protocol;
use bevy::prelude::trace;

use web_transport::{Error, RecvStream, SendStream};
use std::mem::size_of;
use bevy::log::debug;
use futures::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::{SendError, TryRecvError};
use tokio::task::JoinHandle;
use crate::bistream::{read_exact, StreamError};

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

            trace!("[Task Pool] [Writer] Started");

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