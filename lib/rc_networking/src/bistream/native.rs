use crate::Protocol;
use bevy::prelude::trace;

use web_transport::{Error, RecvStream, SendStream};
use std::mem::size_of;
use bevy::log::debug;
use futures::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::{SendError, TryRecvError};
use tokio::task::JoinHandle;
use crate::bistream::{read_exact, recv_protocol, send_protocol, StreamError};

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
                if let Err(e) = send_protocol(&packet, &mut send).await
                {
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
                let data = match recv_protocol(&mut recv).await {
                    Ok(v) => v,
                    Err(e) => {
                        err.send(StreamError::Error).unwrap();
                        trace!("Network stream exited: {:?}", e);
                        return recv;
                    }
                };

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

    pub async fn recv(&mut self) -> Result<Protocol, TryRecvError> {
        match self.in_recv.recv().await {
            Some(v) => Ok(v),
            None => Err(TryRecvError::Disconnected)
        }
    }

    pub fn send(&self, message: Protocol) -> Result<(), SendError<Protocol>> {
        self.out_send.send(message)
    }
}