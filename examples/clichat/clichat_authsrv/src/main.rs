use std::time::Duration;

use anyhow::Result as AnyResult;
use clichat_proto::{GameLoginExtras, LoginData};
use iyes_multiplayer_authsrv::{
    account::{
        AccountVerifier, GameExtrasHandler, VerifyError,
        VerifySuccess, AccountVerifyFuture, GameExtrasProcessFuture,
    },
    server::GameAuthServer,
};
use iyes_multiplayer_proto_clientauth::{
    handshake::{AccountError},
    Never,
};
use tracing::{error, info};

#[derive(Debug)]
struct ChatExtrasHandler {
    // TODO
}

impl GameExtrasHandler for ChatExtrasHandler {
    type Data = GameLoginExtras;
    type Error = Never;
    fn process(
        &mut self,
        data: &Self::Data,
    ) -> GameExtrasProcessFuture<Self> {
        let allow_nsfw = data.allow_nsfw;
        Box::pin(async move {
            // TODO
            info!("Client requested NSFW: {:?}", allow_nsfw);
            Ok(())
        })
    }
}

#[derive(Debug)]
struct SecretWordVerifier;

impl AccountVerifier<LoginData> for SecretWordVerifier {
    type Error = Never;
    fn verify(
        &mut self,
        data: &LoginData,
    ) -> AccountVerifyFuture<LoginData, Self> {
        let word = data.secret_word.clone();
        Box::pin(async move {
            if word == "friends" {
                Ok(VerifySuccess::Success)
            } else {
                Err(VerifyError::AccountError(AccountError::BadCredentials))
            }
        })
    }
}

struct ChatAuthServer;

impl GameAuthServer for ChatAuthServer {
    type AccountData = LoginData;
    type AccountDataError = Never;
    type GameExtras = GameLoginExtras;
    type GameExtrasError = Never;
    type AccountVerifier = SecretWordVerifier;
    type GameExtrasHandler = ChatExtrasHandler;
}

#[tokio::main]
async fn run() -> AnyResult<()> {
    // TODO: do not hardcode everything
    let certs = iyes_multiplayer_authsrv::config::AuthServerCertificates::load_from_dir("certs")?;
    let server = iyes_multiplayer_authsrv::server::AuthServer::<ChatAuthServer>::new(
        certs,
        SecretWordVerifier,
        ChatExtrasHandler {},
    )?;
    server.run("127.0.0.1:23456", "127.0.0.1:12345")?;
    info!("Auth Server up and running!");

    // FIXME: implement shutdown mechanism
    tokio::time::sleep(Duration::from_secs(1000)).await;

    Ok(())
}

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )
    .unwrap();

    if let Err(e) = run() {
        error!("{}", e);
    }
}
