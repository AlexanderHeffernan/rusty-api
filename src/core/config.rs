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

/*
    Load TLS configuration for HTTPS.
    This function reads the certificate and private key from the specified paths.
    It returns a ServerConfig object that can be used to configure the Actix web server.
*/
pub fn load_rustls_config(cert_path: impl AsRef<Path>, key_path: impl AsRef<Path>) -> Result<ServerConfig, ConfigError> {
    let cert_path = cert_path.as_ref();
    let key_path = key_path.as_ref();

    // Load the certificate chain from the provided file
    let cert_chain: Vec<CertificateDer> = CertificateDer::pem_file_iter(cert_path)
        .map_err(|e| ConfigError::Pem(e))?
        .flatten()
        .collect();

    // Ensure at least one certificate is present
    if cert_chain.is_empty() {
        return Err(ConfigError::NoCertificates(cert_path.display().to_string()));
    }

    // Load the private key from the provided file
    let key_der = PrivateKeyDer::from_pem_file(key_path)
        .map_err(|_| ConfigError::NoPrivateKey(key_path.display().to_string()))?;

    // Build and return the Rustls server configuration
    Ok(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
        .expect("Invalid certificate or key"))
}