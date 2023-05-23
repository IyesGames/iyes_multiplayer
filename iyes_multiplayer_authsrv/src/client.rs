use std::time::Duration;

use iyes_multiplayer_proto_clientauth::handshake::{HandshakeRequest, HandshakeResponseSuccess, HandshakeResponseError, AccountData, GameExtras};
use quinn::{Connection, Endpoint};
use thiserror::Error;
use tracing::{error, info};

use crate::{AuthServerError, account::{AccountVerifier, GameExtrasHandler, VerifySuccess, VerifyError, GameError}, server::{GameAuthServer, AuthServerCustom}};

#[derive(Error, Debug)]
enum ClientSessionError<S: GameAuthServer> {
    #[error("Handshake timed out: {0}")]
    HandshakeTimeout(tokio::time::error::Elapsed),
    #[error("Handshake error: {0}")]
    Handshake(#[from] HandshakeError<S>),
}

#[derive(Error, Debug)]
pub enum HandshakeError<S: GameAuthServer> {
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
    #[error("Account verification error: {0}")]
    AccountVerifier(<S::AccountVerifier as AccountVerifier<S::AccountData>>::Error),
    #[error("Game Extras processing error: {0}")]
    GameExtrasHandler(<S::GameExtrasHandler as GameExtrasHandler>::Error),
}

pub(crate) async fn listen_clients<S: GameAuthServer>(server_custom: AuthServerCustom<S>, endpoint: Endpoint) -> Result<(), AuthServerError> {
    if let Ok(addr) = endpoint.local_addr() {
        info!("Listening for Client connections on {}.", addr);
    } else {
        info!("Listening for Client connections on unknown address.");
    }

    while let Some(conn) = endpoint.accept().await {
        let remote_addr = conn.remote_address();

        if let Some(local_addr) = conn.local_ip() {
            info!(
                "Incoming Client connection from {} at local address {}.",
                remote_addr, local_addr
            );
        } else {
            info!("Incoming Client connection from {}.", remote_addr);
        }

        match conn.await {
            Ok(conn) => {
                let server_custom = server_custom.clone();
                tokio::spawn(async move {
                    if let Err(e) = client_connection(server_custom, conn).await {
                        error!("Client session error: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Client connection from {} failed: {}", remote_addr, e);
            }
        }

        // TODO: implement clean shutdown
    }
    Ok(())
}

async fn client_connection<S: GameAuthServer>(server_custom: AuthServerCustom<S>, conn: Connection) -> Result<(), ClientSessionError<S>> {
    tokio::time::timeout(Duration::from_secs(1), recv_handshake(&server_custom, &conn)).await
        .map_err(ClientSessionError::HandshakeTimeout)??;
    Ok(())
}

async fn recv_handshake<S: GameAuthServer>(server_custom: &AuthServerCustom<S>, conn: &Connection) -> Result<(), HandshakeError<S>>
{
    let (mut tx, rx) = conn.accept_bi().await?;
    let buf = rx.read_to_end(256).await?;
    let request: HandshakeRequest<S::AccountData, S::GameExtras> = rmp_serde::from_slice(&buf)?;
    let mut out: Result<
        HandshakeResponseSuccess,
        HandshakeResponseError<
            <S::AccountData as AccountData>::Error,
            <S::GameExtras as GameExtras>::Error
        >
    > = Ok(HandshakeResponseSuccess::AuthWelcome);

    match dbg!(server_custom.account_verifier.lock().await.verify(&request.account_data).await) {
        Ok(VerifySuccess::Success) => {}
        Err(VerifyError::AccountError(e)) => {
            out = Err(HandshakeResponseError::Account(e));
        }
        Err(VerifyError::VerifierError(e)) => {
            return Err(HandshakeError::AccountVerifier(e));
        }
    }

    match server_custom.game_extras_handler.lock().await.process(&request.game_extras).await {
        Ok(()) => {}
        Err(GameError::GameExtrasError(e)) => {
            out = Err(HandshakeResponseError::GameExtras(e));
        }
        Err(GameError::ServerError(e)) => {
            return Err(HandshakeError::GameExtrasHandler(e));
        }
    }

    let out = rmp_serde::to_vec(&out)?;
    tx.write_all(&out).await?;
    tx.finish().await?;
    dbg!(request);
    Ok(())
}
