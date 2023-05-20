use quinn::{Connection, Endpoint};
use thiserror::Error;
use tracing::{error, info};

use crate::AuthServerError;

#[derive(Error, Debug)]
enum HostSessionError {}

pub(crate) async fn listen_hosts(endpoint: Endpoint) -> Result<(), AuthServerError> {
    if let Ok(addr) = endpoint.local_addr() {
        info!("Listening for Host Server connections on {}.", addr);
    } else {
        info!("Listening for Host Server connections on unknown address.");
    }

    while let Some(conn) = endpoint.accept().await {
        let remote_addr = conn.remote_address();

        if let Some(local_addr) = conn.local_ip() {
            info!(
                "Incoming Host Server connection from {} at local address {}.",
                remote_addr, local_addr
            );
        } else {
            info!("Incoming Host Server connection from {}.", remote_addr);
        }

        match conn.await {
            Ok(conn) => {
                tokio::spawn(async move {
                    if let Err(e) = host_connection(conn).await {
                        error!("Host Server session error: {:#}", e);
                    }
                });
            }
            Err(e) => {
                error!(
                    "Host Server connection from {} failed: {:#}",
                    remote_addr, e
                );
            }
        }

        // TODO: implement clean shutdown
    }
    Ok(())
}

async fn host_connection(conn: Connection) -> Result<(), HostSessionError> {
    Ok(())
}
