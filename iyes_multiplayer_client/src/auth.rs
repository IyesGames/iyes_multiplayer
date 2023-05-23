use iyes_multiplayer_proto_clientauth::handshake::{
    HandshakeRequest, HandshakeResponseError, HandshakeResponseSuccess,
};
use quinn::Connection;
use thiserror::Error;

use crate::client::GameClient;

#[derive(Error, Debug)]
pub enum HandshakeError<C: GameClient> {
    #[error("Connection error: {0}")]
    Connection(#[from] quinn::ConnectionError),
    #[error("Message encoding failed: {0}")]
    Serialize(#[from] rmp_serde::encode::Error),
    #[error("Message decoding failed: {0}")]
    Deserialize(#[from] rmp_serde::decode::Error),
    #[error("Error sending data: {0}")]
    Write(#[from] quinn::WriteError),
    #[error("Error receiving data: {0}")]
    Read(#[from] quinn::ReadToEndError),
    #[error("Handshake refused: {0}")]
    Handshake(#[from] HandshakeResponseError<C::AccountDataError, C::GameExtrasError>),
}

pub struct AuthConnection {
    conn: Connection,
}

impl AuthConnection {
    pub(crate) fn new(conn: Connection) -> Self {
        AuthConnection { conn }
    }

    pub(crate) async fn handshake<C: GameClient>(
        &self,
        request: HandshakeRequest<'_, C::AccountData, C::GameExtras>,
    ) -> Result<HandshakeResponseSuccess, HandshakeError<C>> {
        let (mut tx, rx) = self.conn.open_bi().await?;
        let buf = rmp_serde::to_vec_named(&request)?;
        tx.write_all(&buf).await?;
        tx.finish().await?;
        let buf = rx.read_to_end(1024).await?;
        let response: Result<
            HandshakeResponseSuccess,
            HandshakeResponseError<C::AccountDataError, C::GameExtrasError>,
        > = rmp_serde::from_slice(&buf)?;
        Ok(response?)
    }
}
