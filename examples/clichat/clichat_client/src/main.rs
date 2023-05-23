use anyhow::Result as AnyResult;
use clichat_proto::{GameLoginExtras, LoginData};
use iyes_multiplayer_client::client::GameClient;
use iyes_multiplayer_proto_clientauth::Never;
use tracing::{error, info};

#[derive(Debug)]
struct ChatClient;

impl GameClient for ChatClient {
    type AccountData = LoginData;
    type AccountDataError = Never;
    type GameExtras = GameLoginExtras;
    type GameExtrasError = Never;
}

#[tokio::main]
async fn run(name: String, secret_word: String) -> AnyResult<()> {
    println!("Connecting to Auth server…");

    // TODO: do not hardcode everything
    let certs = iyes_multiplayer_client::config::ClientCertificates::load_from_dir("certs")?;
    let client =
        iyes_multiplayer_client::client::ClientBuilder::<ChatClient>::new(certs, (0, 1), (0, 1))?;
    let client = client.with_game_account(
        LoginData { secret_word },
        GameLoginExtras { allow_nsfw: true },
    );
    client
        .connect_auth("auth.iyes.games", "127.0.0.1:12345".parse()?, &name)
        .await?;
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

    println!("Welcome to Just Chatting™!");
    println!("What's your name?");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_owned();
    println!("Hello, {}! You'll be chatting in no time!", name);

    println!(
        "What is the secret code word to be allowed into the chat rooms? (pssst… it's 'friends')"
    );
    let mut secret_word = String::new();
    std::io::stdin().read_line(&mut secret_word).unwrap();
    let secret_word = secret_word.trim().to_owned();

    if let Err(e) = run(name, secret_word) {
        error!("{}", e);
    }
}
