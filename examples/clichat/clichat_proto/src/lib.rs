use iyes_multiplayer_proto_clientauth::{handshake::{AccountData, GameExtras}, Never};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginData {
    pub secret_word: String,
}

impl AccountData for LoginData {
    type Error = Never;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameLoginExtras {
    pub allow_nsfw: bool,
}

impl GameExtras for GameLoginExtras {
    type Error = Never;
}
