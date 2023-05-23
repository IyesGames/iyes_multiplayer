use client::GameClient;
use thiserror::Error;

pub mod client;
pub mod config;

pub mod auth;

#[derive(Error, Debug)]
pub enum ClientError<C: GameClient> {
    #[error("Bad certificate: {0}")]
    Certificate(webpki::Error),
    #[error("Could not configure client TLS cryptography: {0}")]
    ClientCrypto(rustls::Error),
    #[error("Could not set up QUIC endpoint: {0}")]
    Endpoint(std::io::Error),
    #[error("Cannot attempt to connect to server: {0}")]
    Connect(#[from] quinn::ConnectError),
    #[error("Connection failed: {0}")]
    Connection(#[from] quinn::ConnectionError),
    #[error("Auth Server Handshake failed: {0}")]
    Handshake(#[from] crate::auth::HandshakeError<C>),
}
