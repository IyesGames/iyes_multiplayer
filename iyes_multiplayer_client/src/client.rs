use std::{marker::PhantomData, net::SocketAddr, sync::Arc};

use iyes_multiplayer_proto_clientauth::handshake::{
    AccountData, AccountDataError, GameExtras, GameExtrasError, HandshakeRequest,
};
use quinn::Endpoint;
use rustls::{ClientConfig, RootCertStore};

use crate::{auth::AuthConnection, config::ClientCertificates, ClientError};

/// Configuration trait: impl this to customize the Client protocol for your game/application
pub trait GameClient: Send + Sync + Sized + 'static {
    /// The Account Data to expect in protocol handshake messages
    type AccountData: AccountData<Error = Self::AccountDataError> + serde::Serialize;
    /// The Game Extras data to expect in protocol handshake messages
    type GameExtras: GameExtras<Error = Self::GameExtrasError> + serde::Serialize;

    type AccountDataError: AccountDataError + serde::de::DeserializeOwned;
    type GameExtrasError: GameExtrasError + serde::de::DeserializeOwned;
}

pub struct ClientBuilder<C: GameClient> {
    crypto_auth: Arc<ClientConfig>,
    endpoint: Endpoint,
    ver_proto: (u8, u8),
    ver_client: (u8, u8),
    _data: PhantomData<C>,
}

impl<C: GameClient> ClientBuilder<C> {
    pub fn new(
        certs: ClientCertificates,
        ver_proto: (u8, u8),
        ver_client: (u8, u8),
    ) -> Result<Self, ClientError<C>> {
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

        Ok(ClientBuilder {
            crypto_auth,
            endpoint,
            ver_proto,
            ver_client,
            _data: PhantomData,
        })
    }

    pub fn with_game_account(
        self,
        account_data: C::AccountData,
        game_extras: C::GameExtras,
    ) -> Client<C> {
        Client {
            builder: self,
            account_data,
            game_extras,
        }
    }
}

pub struct Client<C: GameClient> {
    builder: ClientBuilder<C>,
    account_data: C::AccountData,
    game_extras: C::GameExtras,
}

impl<C: GameClient> Client<C> {
    pub async fn connect_auth(
        &self,
        server_name: &str,
        server_addr: SocketAddr,
        user_display_name: &str,
    ) -> Result<AuthConnection, ClientError<C>> {
        let client_config = quinn::ClientConfig::new(self.builder.crypto_auth.clone());
        let conn = self
            .builder
            .endpoint
            .connect_with(client_config, server_addr, server_name)?
            .await?;
        let conn = AuthConnection::new(conn);

        let response = conn
            .handshake::<C>(HandshakeRequest {
                proto_version_major: self.builder.ver_proto.0,
                proto_version_minor: self.builder.ver_proto.1,
                client_version_major: self.builder.ver_client.0,
                client_version_minor: self.builder.ver_client.1,
                display_name: user_display_name,
                account_data: self.account_data.clone(),
                game_extras: self.game_extras.clone(),
            })
            .await
            .map_err(ClientError::Handshake)?;

        dbg!(response);

        Ok(conn)
    }
}
