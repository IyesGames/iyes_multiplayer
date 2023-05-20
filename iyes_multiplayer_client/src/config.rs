use std::path::Path;

use rustls::{Certificate, PrivateKey};

/// Certificates and keys needed for the Auth Server to operate
#[derive(Clone)]
pub struct ClientCertificates {
    /// Master certificate, used to verify connections to Auth and Host servers, signer of ClientAuth
    pub master_cert: Certificate,
    /// ClientAuth certificate, signer of the Client cert
    pub clientauth_cert: Certificate,
    /// Client certificate, used to connect to Auth server
    pub client_cert: Certificate,
    /// Client certificate private key
    pub client_key: PrivateKey,
}

impl ClientCertificates {
    /// Loads everything from a directory, assuming standard naming scheme
    pub fn load_from_dir(dir: impl AsRef<Path>) -> std::io::Result<ClientCertificates> {
        let dir = dir.as_ref();
        let master_cert = std::fs::read(dir.join("master.cert.der"))?;
        let clientauth_cert = std::fs::read(dir.join("clientauth.cert.der"))?;
        let client_cert = std::fs::read(dir.join("client.cert.der"))?;
        let client_key = std::fs::read(dir.join("client.key.der"))?;
        Ok(ClientCertificates {
            master_cert: Certificate(master_cert),
            clientauth_cert: Certificate(clientauth_cert),
            client_cert: Certificate(client_cert),
            client_key: PrivateKey(client_key),
        })
    }
}
