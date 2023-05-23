use std::{net::ToSocketAddrs, sync::Arc};

use iyes_multiplayer_proto_clientauth::{handshake::{AccountData, GameExtras, GameExtrasError, AccountDataError}};
use rustls::{server::AllowAnyAuthenticatedClient, RootCertStore, ServerConfig};
use tokio::sync::Mutex;

use crate::{config::AuthServerCertificates, AuthServerError, account::{AccountVerifier, GameExtrasHandler}};

/// Configuration trait: impl this to customize the Auth Server and its protocol for your game/application
pub trait GameAuthServer: Send + Sync + Sized + 'static {
    /// The Account Data to expect in protocol handshake messages
    type AccountData: AccountData<Error = Self::AccountDataError> + serde::de::DeserializeOwned;
    /// The backend for verifying accounts during protocol handshake
    type AccountVerifier: AccountVerifier<Self::AccountData>;
    /// The Game Extras data to expect in protocol handshake messages
    type GameExtras: GameExtras<Error = Self::GameExtrasError> + serde::de::DeserializeOwned;
    /// The backend for handling the game extras in handshakes
    type GameExtrasHandler: GameExtrasHandler<Data = Self::GameExtras>;

    type AccountDataError: AccountDataError + serde::Serialize;
    type GameExtrasError: GameExtrasError + serde::Serialize;
}

pub(crate) struct AuthServerCustom<S: GameAuthServer> {
    pub(crate) account_verifier: Arc<Mutex<S::AccountVerifier>>,
    pub(crate) game_extras_handler: Arc<Mutex<S::GameExtrasHandler>>,
}

impl<S: GameAuthServer> Clone for AuthServerCustom<S> {
    fn clone(&self) -> Self {
        Self {
            account_verifier: self.account_verifier.clone(),
            game_extras_handler: self.game_extras_handler.clone(),
        }
    }
}

pub struct AuthServer<S: GameAuthServer> {
    server_custom: AuthServerCustom<S>,
    crypto_host: Arc<ServerConfig>,
    crypto_client: Arc<ServerConfig>,
}

impl<S: GameAuthServer> AuthServer<S> {
    pub fn new(certs: AuthServerCertificates, account_verifier: S::AccountVerifier, game_extras_handler: S::GameExtrasHandler) -> Result<Self, AuthServerError> {
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
            server_custom: AuthServerCustom {
                account_verifier: Arc::new(Mutex::new(account_verifier)),
                game_extras_handler: Arc::new(Mutex::new(game_extras_handler)),
            },
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
            tokio::spawn(crate::client::listen_clients(self.server_custom.clone(), endpoint));
        }
        Ok(())
    }
}
