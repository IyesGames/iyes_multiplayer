use thiserror::Error;

pub mod client;
pub mod config;

pub mod auth;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("bad certificate")]
    Certificate(webpki::Error),
    #[error("could not configure client TLS cryptography")]
    ClientCrypto(rustls::Error),
    #[error("could not set up QUIC endpoint")]
    Endpoint(std::io::Error),
    #[error("cannot attempt to connect to server")]
    Connect(#[from] quinn::ConnectError),
    #[error("connection failed")]
    Connection(#[from] quinn::ConnectionError),
}
