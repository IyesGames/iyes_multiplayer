use std::{net::ToSocketAddrs, sync::Arc};

use rustls::{server::AllowAnyAuthenticatedClient, RootCertStore, ServerConfig};

use crate::{config::AuthServerCertificates, AuthServerError};

pub struct AuthServer {
    crypto_host: Arc<ServerConfig>,
    crypto_client: Arc<ServerConfig>,
}

impl AuthServer {
    pub fn new(certs: AuthServerCertificates) -> Result<Self, AuthServerError> {
        // Configure the crypto for listening for hosts
        let mut host_roots = RootCertStore::empty();
        host_roots
            .add(&certs.hostauth_cert)
            .map_err(AuthServerError::Certificate)?;
        let crypto_host = Arc::new(
            rustls::ServerConfig::builder()
                .with_safe_defaults()
                .with_client_cert_verifier(AllowAnyAuthenticatedClient::new(host_roots))
                .with_single_cert(
                    vec![certs.authsrv_cert.clone(), certs.master_cert.clone()],
                    certs.authsrv_key.clone(),
                )
                .map_err(AuthServerError::ServerCrypto)?,
        );

        // Configure the crypto for listening for clients
        let mut client_roots = RootCertStore::empty();
        client_roots
            .add(&certs.clientauth_cert)
            .map_err(AuthServerError::Certificate)?;
        let crypto_client = Arc::new(
            rustls::ServerConfig::builder()
                .with_safe_defaults()
                .with_client_cert_verifier(AllowAnyAuthenticatedClient::new(client_roots))
                .with_single_cert(
                    vec![certs.authsrv_cert.clone(), certs.master_cert.clone()],
                    certs.authsrv_key.clone(),
                )
                .map_err(AuthServerError::ServerCrypto)?,
        );

        Ok(AuthServer {
            crypto_host,
            crypto_client,
        })
    }

    pub fn run(
        self,
        host_listen_addrs: impl ToSocketAddrs,
        client_listen_addrs: impl ToSocketAddrs,
    ) -> Result<(), AuthServerError> {
        // Create an endpoint for each address and listen
        for addr in host_listen_addrs
            .to_socket_addrs()
            .map_err(AuthServerError::Endpoint)?
        {
            let server_config = quinn::ServerConfig::with_crypto(self.crypto_host.clone());
            let endpoint =
                quinn::Endpoint::server(server_config, addr).map_err(AuthServerError::Endpoint)?;
            tokio::spawn(crate::host::listen_hosts(endpoint));
        }

        // Create an endpoint for each address and listen
        for addr in client_listen_addrs
            .to_socket_addrs()
            .map_err(AuthServerError::Endpoint)?
        {
            let server_config = quinn::ServerConfig::with_crypto(self.crypto_client.clone());
            let endpoint =
                quinn::Endpoint::server(server_config, addr).map_err(AuthServerError::Endpoint)?;
            tokio::spawn(crate::client::listen_clients(endpoint));
        }
        Ok(())
    }
}
