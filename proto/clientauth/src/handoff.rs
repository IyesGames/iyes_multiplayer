use std::net::SocketAddr;

use serde::{Serialize, Deserialize};

/// Info needed to perform hand-off (connect to Host server)
#[derive(Serialize, Deserialize, Debug)]
pub struct HandOffData {
    pub session_id: u64,
    pub client_session_nonce: u64,
    pub host_addr: SocketAddr,
    pub host_name: String,
    pub session_cert: Vec<u8>,
    pub session_key: Vec<u8>,
}

