// Cryptographic primitives and utilities

pub mod hkdf;

use crate::error::Result;

/// Represents encrypted data with metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedData {
    /// The encrypted ciphertext
    pub ciphertext: Vec<u8>,
    
    /// Metadata about encryption layers used
    pub layers: Vec<String>,
    
    /// Version of HybridGuard used
    pub version: String,
    
    /// Timestamp of encryption
    pub timestamp: u64,
}

impl EncryptedData {
    pub fn new(ciphertext: Vec<u8>) -> Self {
        Self {
            ciphertext,
            layers: vec![
                "ML-KEM-768".to_string(),
                "HQC".to_string(),
                "QuantumNoise".to_string(),
                "FHE".to_string(),
            ],
            version: "0.1.0".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
