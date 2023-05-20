use thiserror::Error;

pub mod config;
pub mod server;

mod client;
mod host;

#[derive(Error, Debug)]
pub enum AuthServerError {
    #[error("bad certificate")]
    Certificate(webpki::Error),
    #[error("could not configure server TLS cryptography")]
    ServerCrypto(rustls::Error),
    #[error("could not set up QUIC endpoint")]
    Endpoint(std::io::Error),
}
