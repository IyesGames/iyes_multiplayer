use std::{error::Error, fmt::Debug, future::Future, pin::Pin};

use iyes_multiplayer_proto_clientauth::{
    handshake::{AccountData, AccountDataError, AccountError, GameExtras, GameExtrasError},
    Never,
};
use thiserror::Error;

pub type AccountVerifyFuture<D, A> = Pin<
    Box<
        dyn Future<
            Output = Result<
                VerifySuccess,
                VerifyError<<D as AccountData>::Error, <A as AccountVerifier<D>>::Error>,
            >,
        > + Send,
    >,
>;

pub trait AccountVerifier<D: AccountData>: Debug + Send + Sync + Sized {
    type Error: Error;

    fn verify(&mut self, data: &D) -> AccountVerifyFuture<D, Self>;
}

#[derive(Error, Debug)]
pub enum VerifyError<AE: AccountDataError, VE: Error> {
    /// Error with the account data (client-side error)
    #[error("Bad account data provided by client")]
    AccountError(AccountError<AE>),
    /// Error with the verifier backend (server-side error)
    #[error("Account could not be verified")]
    VerifierError(VE),
}

#[derive(Debug)]
pub enum VerifySuccess {
    Success,
}

#[derive(Debug)]
pub struct NullVerifier;

impl<D: AccountData> AccountVerifier<D> for NullVerifier {
    type Error = Never;
    fn verify(
        &mut self,
        _data: &D,
    ) -> Pin<
        Box<dyn Future<Output = Result<VerifySuccess, VerifyError<D::Error, Self::Error>>> + Send>,
    > {
        Box::pin(async move { Ok(VerifySuccess::Success) })
    }
}

pub type GameExtrasProcessFuture<G> = Pin<
    Box<
        dyn Future<
            Output = Result<
                (),
                GameError<
                    <<G as GameExtrasHandler>::Data as GameExtras>::Error,
                    <G as GameExtrasHandler>::Error,
                >,
            >,
        > + Send,
    >,
>;

pub trait GameExtrasHandler: Debug + Send + Sync + Sized {
    type Data: GameExtras;
    type Error: Error;

    fn process(&mut self, data: &Self::Data) -> GameExtrasProcessFuture<Self>;
}

#[derive(Error, Debug)]
pub enum GameError<GE: GameExtrasError, SE: Error> {
    /// Error with data (client-side error)
    #[error("Bad game extras data provided by client")]
    GameExtrasError(GE),
    /// Error with handler (server-side error)
    #[error("Game extras could not be processed by server")]
    ServerError(SE),
}

#[derive(Debug)]
pub struct NullGameExtrasHandler;

impl GameExtrasHandler for NullGameExtrasHandler {
    type Data = ();
    type Error = Never;
    fn process(
        &mut self,
        _data: &Self::Data,
    ) -> Pin<Box<dyn Future<Output = Result<(), GameError<Never, Self::Error>>> + Send>> {
        Box::pin(async move { Ok(()) })
    }
}
