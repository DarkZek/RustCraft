use bevy::prelude::debug;
use byteorder::{BigEndian, WriteBytesExt};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use web_transport::{Error, RecvStream, Session};
use rc_shared::constants::UserId;
use crate::bistream::{BiStream, recv_protocol, send_protocol, StreamError};
use crate::protocol::Protocol;

pub struct HandshakeResult {
    pub unreliable: BiStream,
    pub reliable: BiStream,
    pub chunk: BiStream,
    pub err_recv: UnboundedReceiver<StreamError>
}

/// Negotiates a set of streams, and user id with the server
pub async fn negotiate_handshake(session: &mut Session, join_token: String) -> Result<HandshakeResult, Error> {

    let mut unreliable = session.accept_bi().await.unwrap();
    let mut reliable = session.accept_bi().await.unwrap();
    let mut chunk = session.accept_bi().await.unwrap();

    debug!("Accepted bi streams");

    // Channel must send data to be created, so verify data sent and remove from reader
    async fn verify_stream(stream: &mut RecvStream, expected: &str) -> Result<(), Error> {
        let bytes = stream.read(5).await?.unwrap();
        let contents = String::from_utf8(bytes.to_vec()).unwrap();

        if contents != expected {
            panic!(
                "Invalid client attempted connection. Contents: {} [{:?}] Expected: {} [{:?}]",
                contents,
                contents.as_bytes(),
                expected,
                expected.as_bytes(),
            );
        }

        Ok(())
    }

    verify_stream(&mut unreliable.1, "Test1").await?;
    verify_stream(&mut reliable.1, "Test2").await?;
    verify_stream(&mut chunk.1, "Test3").await?;

    debug!("Verified streams");

    send_protocol(&Protocol::Authorization(join_token), &mut reliable.0).await.unwrap();
    let response = recv_protocol(&mut reliable.1).await.unwrap();
    assert_eq!(response, Protocol::AuthorizationAccepted);

    debug!("Provided authentication");

    let (send_err, err_recv) = unbounded_channel();

    let unreliable = BiStream::from_stream(unreliable.0, unreliable.1, send_err.clone());
    let mut reliable = BiStream::from_stream(reliable.0, reliable.1, send_err.clone());
    let chunk = BiStream::from_stream(chunk.0, chunk.1, send_err);

    debug!("Created bi streams");

    Ok(HandshakeResult {
        unreliable,
        reliable,
        chunk,
        err_recv
    })
}