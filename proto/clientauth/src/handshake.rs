use std::error::Error;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::Never;
use crate::handoff::HandOffData;

/// Must be sent by client immediately after establishing QUIC connection
///
/// Auth server will respond with a `Result<HandshakeResponseSuccess, HandshakeResponseError>`.
#[derive(Serialize, Deserialize, Debug)]
pub struct HandshakeRequest<'a, A: AccountData, G: GameExtras> {
    pub proto_version_major: u8,
    pub proto_version_minor: u8,
    pub client_version_major: u8,
    pub client_version_minor: u8,
    // FIXME: these can be a DDOS vector; replace with something safe
    pub display_name: &'a str,
    pub account_data: A,
    pub game_extras: G,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HandshakeResponseSuccess {
    /// Auth server accepts the client, stay connected and talk to the Auth! :)
    AuthWelcome,
    /// Immediate Hand-Off; client must connect to host immediately
    /// Useful for quickly rejoining game matches after client crash, for example.
    HandOffNow(HandOffData),
}

#[derive(Serialize, Deserialize, Error, Debug)]
pub enum HandshakeResponseError<AE: AccountDataError, GE: GameExtrasError> {
    /// Client version or proto version too old
    #[error("Unsupported version. Too old. Please update client app.")]
    VersionTooOld,
    /// Client version or proto version too new
    #[error("Unsupported version. Too new. Using old servers?")]
    VersionTooNew,
    /// Account verification failed, valid account required
    #[error("Account verification failed: {0}")]
    Account(#[from] AccountError<AE>),
    #[error("Bad GameExtras: {0}")]
    GameExtras(#[from] GE),
}

#[derive(Serialize, Deserialize, Error, Debug)]
pub enum AccountError<E: AccountDataError> {
    /// Account does not exist
    #[error("Account does not exist")]
    NoSuchAccount,
    /// Provided credentials are invalid
    #[error("Wrong credentials")]
    BadCredentials,
    /// Account banned from multiplayer services
    #[error("You are banned from multiplayer services!")]
    Banned,
    /// Other (custom) error
    #[error("Other error: {0}")]
    Other(E),
}

pub trait AccountData: Debug + Clone + Sized + Send + Sync {
    type Error: AccountDataError;
}
pub trait GameExtras: Debug + Clone + Sized + Send + Sync {
    type Error: GameExtrasError;
}
pub trait AccountDataError: Debug + Error + Sized + Send + Sync {}
pub trait GameExtrasError: Debug + Error + Sized + Send + Sync {}

impl AccountData for () {
    type Error = Never;
}
impl GameExtras for () {
    type Error = Never;
}
impl AccountDataError for Never {}
impl GameExtrasError for Never {}

