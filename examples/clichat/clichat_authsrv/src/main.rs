use std::time::Duration;

use anyhow::{Result as AnyResult};
use tracing::{error, info};

#[tokio::main]
async fn run() -> AnyResult<()> {
    // TODO: do not hardcode everything
    let certs = iyes_multiplayer_authsrv::config::AuthServerCertificates::load_from_dir("certs")?;
    let server = iyes_multiplayer_authsrv::server::AuthServer::new(certs)?;
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
        error!("{:#}", e);
    }
}
