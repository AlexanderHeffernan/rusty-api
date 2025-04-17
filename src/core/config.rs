/*!
 * The `config` module provides functionality for loading and configuring TLS settings for the API server.
 *
 * This module simplifies the process of setting up secure HTTPS communication by
 * reading certificate and private key files and creating a `ServerConfig` object
 * compatible with Rustls and Actix Web. This module is used internally by the `Api` struct.
 *
 * This module features:
 * - **Certificate Loading**: Reads and parses PEM-encoded certificate chains.
 * - **Private Key Loading**: Reads and parses PEM-encoded private keys.
 * - **Rustls Integration**: Creates a `ServerConfig` for secure HTTPS communication.
 *
 * # Example
 * ```rust,no_run
 * use rusty_api::Api;
 *
 * let api = Api::new()
 *    .certs("path/to/cert.pem", "path/to/key.pem") // Load the certificate and key into the API struct
 *    .start(); // Starting the API server will call this module internally.
 * ```
 */
use rustls::{pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject}, ServerConfig};
use std::path::Path;

/// Loads the TLS configuration for the API server.
pub fn load_rustls_config(cert_path: impl AsRef<Path>, key_path: impl AsRef<Path>) -> Option<ServerConfig> {
    let cert_path = cert_path.as_ref();
    let key_path = key_path.as_ref();

    // Load the certificate chain from the provided file
    let cert_chain: Vec<CertificateDer> = match CertificateDer::pem_file_iter(cert_path)
        .map(|res| res.flatten().collect::<Vec<_>>())
    {
        Ok(chain) if !chain.is_empty() => chain,
        Ok(_) => {
            println!("Error: No certificates found in {}", cert_path.display());
            return None;
        }
        Err(e) => {
            println!("Error: Failed to parse PEM file at {}: {}", cert_path.display(), e);
            return None;
        }
    };

    // Load the private key from the provided file
    let key_der = match PrivateKeyDer::from_pem_file(key_path) {
        Ok(key) => key,
        Err(_) => {
            println!("Error: No private key found in {}", key_path.display());
            return None;
        }
    };

    // Build and return the Rustls server configuration
    match ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
    {
        Ok(config) => Some(config),
        Err(e) => {
            println!("Error: Failed to build TLS configuration: {}", e);
            None
        }
    }
}