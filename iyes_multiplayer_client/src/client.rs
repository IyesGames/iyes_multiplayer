use std::{net::SocketAddr, sync::Arc};

use quinn::Endpoint;
use rustls::{ClientConfig, RootCertStore};

use crate::{auth::AuthConnection, config::ClientCertificates, ClientError};

pub struct Client {
    crypto_auth: Arc<ClientConfig>,
    endpoint: Endpoint,
}

impl Client {
    pub fn new(certs: ClientCertificates) -> Result<Self, ClientError> {
        // Configure the crypto for connecting to Auth server
        let mut roots = RootCertStore::empty();
        roots
            .add(&certs.master_cert)
            .map_err(ClientError::Certificate)?;
        let crypto_auth = Arc::new(
            rustls::ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(roots)
                .with_single_cert(
                    vec![
                        certs.client_cert.clone(),
                        certs.clientauth_cert.clone(),
                        certs.master_cert.clone(),
                    ],
                    certs.client_key.clone(),
                )
                .map_err(ClientError::ClientCrypto)?,
        );

        // QUIC client endpoint for all outgoing connections (Auth and Host)
        let local_addr = SocketAddr::from(([0, 0, 0, 0], 0));
        let endpoint = quinn::Endpoint::client(local_addr).map_err(ClientError::Endpoint)?;

        Ok(Client {
            crypto_auth,
            endpoint,
        })
    }

    pub async fn connect_auth(
        &self,
        name: &str,
        addr: SocketAddr,
    ) -> Result<AuthConnection, ClientError> {
        let client_config = quinn::ClientConfig::new(self.crypto_auth.clone());
        let conn = self
            .endpoint
            .connect_with(client_config, addr, name)?
            .await?;
        Ok(AuthConnection::new(conn))
    }
}
