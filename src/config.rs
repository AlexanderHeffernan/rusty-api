use rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
use rustls::ServerConfig;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("PEM parsing error: {0}")]
    Pem(#[from] rustls::pki_types::pem::Error),
    #[error("No certificates found in {0}")]
    NoCertificates(String),
    #[error("No private key found in {0}")]
    NoPrivateKey(String),
}

pub fn load_rustls_config(cert_path: impl AsRef<Path>, key_path: impl AsRef<Path>) -> Result<ServerConfig, ConfigError> {
    let cert_path = cert_path.as_ref();
    let key_path = key_path.as_ref();

    let cert_chain: Vec<CertificateDer> = CertificateDer::pem_file_iter(cert_path)
        .map_err(|e| ConfigError::Pem(e))?
        .flatten()
        .collect();
    if cert_chain.is_empty() {
        return Err(ConfigError::NoCertificates(cert_path.display().to_string()));
    }

    let key_der = PrivateKeyDer::from_pem_file(key_path)
        .map_err(|_| ConfigError::NoPrivateKey(key_path.display().to_string()))?;

    Ok(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
        .expect("Invalid certificate or key"))
}