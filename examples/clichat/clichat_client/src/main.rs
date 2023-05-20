use anyhow::{Result as AnyResult};
use tracing::{error, info};

#[tokio::main]
async fn run() -> AnyResult<()> {
    println!("Connecting to Auth server…");

    // TODO: do not hardcode everything
    let certs = iyes_multiplayer_client::config::ClientCertificates::load_from_dir("certs")?;
    let client = iyes_multiplayer_client::client::Client::new(certs)?;
    client.connect_auth("auth.iyes.games", "127.0.0.1:12345".parse()?).await?;
    info!("Auth Server connection successful!");
    Ok(())
}

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )
    .unwrap(); 

    info!("Welcome to Just Chatting™!");

    if let Err(e) = run() {
        error!("{:#}", e);
    }
}
