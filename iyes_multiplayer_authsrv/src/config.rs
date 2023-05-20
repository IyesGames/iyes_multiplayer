use std::path::Path;

use rustls::{Certificate, PrivateKey};

/// Certificates and keys needed for the Auth Server to operate
#[derive(Clone)]
pub struct AuthServerCertificates {
    /// Master certificate, signer of Authsrv, HostAuth, SessionAuth, ClientAuth
    pub master_cert: Certificate,
    /// ClientAuth certificate, used to verify incoming connections from game clients
    pub clientauth_cert: Certificate,
    /// HostAuth certificate, used to verify incoming connections from Host Servers
    pub hostauth_cert: Certificate,
    /// Authsrv (this server's) certificate, used when listening for incoming connections
    pub authsrv_cert: Certificate,
    /// Authsrv certificate private key
    pub authsrv_key: PrivateKey,
    /// SessionAuth certificate, used by this server to sign Session certificates for clients
    pub sessionauth_cert: Certificate,
    /// SessionAuth certificate private key
    pub sessionauth_key: PrivateKey,
}

impl AuthServerCertificates {
    /// Loads everything from a directory, assuming standard naming scheme
    pub fn load_from_dir(dir: impl AsRef<Path>) -> std::io::Result<AuthServerCertificates> {
        let dir = dir.as_ref();
        let master_cert = std::fs::read(dir.join("master.cert.der"))?;
        let clientauth_cert = std::fs::read(dir.join("clientauth.cert.der"))?;
        let hostauth_cert = std::fs::read(dir.join("hostauth.cert.der"))?;
        let authsrv_cert = std::fs::read(dir.join("authsrv.cert.der"))?;
        let authsrv_key = std::fs::read(dir.join("authsrv.key.der"))?;
        let sessionauth_cert = std::fs::read(dir.join("sessionauth.cert.der"))?;
        let sessionauth_key = std::fs::read(dir.join("sessionauth.key.der"))?;
        Ok(AuthServerCertificates {
            master_cert: Certificate(master_cert),
            clientauth_cert: Certificate(clientauth_cert),
            hostauth_cert: Certificate(hostauth_cert),
            authsrv_cert: Certificate(authsrv_cert),
            authsrv_key: PrivateKey(authsrv_key),
            sessionauth_cert: Certificate(sessionauth_cert),
            sessionauth_key: PrivateKey(sessionauth_key),
        })
    }
}
