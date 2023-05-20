use quinn::{Connection, Endpoint};
use thiserror::Error;
use tracing::{error, info};

use crate::AuthServerError;

#[derive(Error, Debug)]
enum ClientSessionError {}

pub(crate) async fn listen_clients(endpoint: Endpoint) -> Result<(), AuthServerError> {
    if let Ok(addr) = endpoint.local_addr() {
        info!("Listening for Client connections on {}.", addr);
    } else {
        info!("Listening for Client connections on unknown address.");
    }

    while let Some(conn) = endpoint.accept().await {
        let remote_addr = conn.remote_address();

        if let Some(local_addr) = conn.local_ip() {
            info!(
                "Incoming Client connection from {} at local address {}.",
                remote_addr, local_addr
            );
        } else {
            info!("Incoming Client connection from {}.", remote_addr);
        }

        match conn.await {
            Ok(conn) => {
                tokio::spawn(async move {
                    if let Err(e) = client_connection(conn).await {
                        error!("Client session error: {:#}", e);
                    }
                });
            }
            Err(e) => {
                error!("Client connection from {} failed: {:#}", remote_addr, e);
            }
        }

        // TODO: implement clean shutdown
    }
    Ok(())
}

async fn client_connection(conn: Connection) -> Result<(), ClientSessionError> {
    Ok(())
}
