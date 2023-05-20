use std::path::PathBuf;

use anyhow::Result as AnyResult;
use rcgen::{BasicConstraints, Certificate, CertificateParams, IsCa};

pub struct IyesMpCertConfig {
    pub name_authsrv: String,
    pub name_hostsrv: String,
    pub certdir: PathBuf,
}

fn gen_cert_master() -> AnyResult<Certificate> {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    Ok(Certificate::from_params(params)?)
}

fn gen_cert_hostauth() -> AnyResult<Certificate> {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::Ca(BasicConstraints::Constrained(1));
    Ok(Certificate::from_params(params)?)
}

fn gen_cert_sessionauth() -> AnyResult<Certificate> {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::Ca(BasicConstraints::Constrained(1));
    Ok(Certificate::from_params(params)?)
}

fn gen_cert_clientauth() -> AnyResult<Certificate> {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::Ca(BasicConstraints::Constrained(1));
    Ok(Certificate::from_params(params)?)
}

fn gen_cert_authsrv(name: &str) -> AnyResult<Certificate> {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::ExplicitNoCa;
    params
        .subject_alt_names
        .push(rcgen::SanType::DnsName(name.to_owned()));
    Ok(Certificate::from_params(params)?)
}

fn gen_cert_hostsrv(name: &str) -> AnyResult<Certificate> {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::ExplicitNoCa;
    params
        .subject_alt_names
        .push(rcgen::SanType::DnsName(name.to_owned()));
    Ok(Certificate::from_params(params)?)
}

fn gen_cert_client() -> AnyResult<Certificate> {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::ExplicitNoCa;
    Ok(Certificate::from_params(params)?)
}

pub fn gen_certs(config: &IyesMpCertConfig) -> AnyResult<()> {
    let master = gen_cert_master()?;
    let hostauth = gen_cert_hostauth()?;
    let sessionauth = gen_cert_sessionauth()?;
    let clientauth = gen_cert_clientauth()?;
    let authsrv = gen_cert_authsrv(&config.name_authsrv)?;
    let hostsrv = gen_cert_hostsrv(&config.name_hostsrv)?;
    let client = gen_cert_client()?;

    std::fs::create_dir_all(&config.certdir)?;
    std::fs::write(
        config.certdir.join("master.cert.der"),
        master.serialize_der()?,
    )?;
    std::fs::write(
        config.certdir.join("master.key.der"),
        master.serialize_private_key_der(),
    )?;
    std::fs::write(
        config.certdir.join("hostauth.cert.der"),
        hostauth.serialize_der_with_signer(&master)?,
    )?;
    std::fs::write(
        config.certdir.join("hostauth.key.der"),
        hostauth.serialize_private_key_der(),
    )?;
    std::fs::write(
        config.certdir.join("sessionauth.cert.der"),
        sessionauth.serialize_der_with_signer(&master)?,
    )?;
    std::fs::write(
        config.certdir.join("sessionauth.key.der"),
        sessionauth.serialize_private_key_der(),
    )?;
    std::fs::write(
        config.certdir.join("clientauth.cert.der"),
        clientauth.serialize_der_with_signer(&master)?,
    )?;
    std::fs::write(
        config.certdir.join("clientauth.key.der"),
        clientauth.serialize_private_key_der(),
    )?;
    std::fs::write(
        config.certdir.join("authsrv.cert.der"),
        authsrv.serialize_der_with_signer(&master)?,
    )?;
    std::fs::write(
        config.certdir.join("authsrv.key.der"),
        authsrv.serialize_private_key_der(),
    )?;
    std::fs::write(
        config.certdir.join("hostsrv.cert.der"),
        hostsrv.serialize_der_with_signer(&master)?,
    )?;
    std::fs::write(
        config.certdir.join("hostsrv.key.der"),
        hostsrv.serialize_private_key_der(),
    )?;
    std::fs::write(
        config.certdir.join("client.cert.der"),
        client.serialize_der_with_signer(&clientauth)?,
    )?;
    std::fs::write(
        config.certdir.join("client.key.der"),
        client.serialize_private_key_der(),
    )?;

    Ok(())
}
