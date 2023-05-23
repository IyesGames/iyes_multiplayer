use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::handoff::HandOffData;

pub mod handoff;
pub mod handshake;
pub mod matchmaking;

/// Message from Client to Auth
///
/// These can be sent freely after a `HandshakeResponseSuccess::AuthWelcome`.
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMsg {
    /// Client is interested in joining a game.
    /// The Auth will begin preparing a session.
    /// Client must wait for `SessionReady` message and then respond with `ConfirmSession`.
    /// Can be cancelled with `CancelSession`.
    SearchSession,

    /// Client is no longer interested in joining a game.
    /// If the Auth had prepared a session, this is treated as an "un-ready"/rejection.
    /// If the Auth is still searching, this is treated as a "stop matchmaking" request.
    CancelSession,

    /// Client confirms ready for hand-off.
    /// Only valid after a `SessionReady`, ignored otherwise.
    ConfirmSession,
}

/// Message from Auth to Client
///
/// These can be received at any time after a `HandshakeResponseSuccess::AuthWelcome`.
#[derive(Serialize, Deserialize, Debug)]
pub enum AuthMsg {
    /// Bad message from client
    MsgError(ClientMsgError),
    /// Client must disconnect from Auth and perform hand-off immediately
    HandOffNow(HandOffData),
    /// Session is ready. Client must accept with a `ConfirmSession`.
    SessionReady,
    /// Client is being kicked, must not attempt new connection until timeout
    Kick(Duration),
}

/// Auth Server is unhappy with a ClientMsg it received.
#[derive(Serialize, Deserialize, Error, Debug)]
pub enum ClientMsgError {}

/// Replacement for the Never type
#[derive(Serialize, Deserialize, Error, Debug)]
pub enum Never {}
