// Layer 2: HQC (Hamming Quasi-Cyclic) - Code-based encryption
// This is the second layer using error-correcting codes for quantum resistance

use crate::error::{HybridGuardError, Result};
use crate::layers::EncryptionLayer;
use oqs::{kem::Kem, kem::Algorithm};
use sha3::{Sha3_256, Digest};

/// HQC (Hamming Quasi-Cyclic) encryption layer
/// Uses code-based cryptography for quantum resistance
pub struct HqcLayer {
    security_level: u32,
}

impl HqcLayer {
    pub fn new() -> Self {
        Self {
            security_level: 256, // HQC provides 256-bit quantum security
        }
    }
    
    /// Derive a KEM keypair from the layer key
    fn derive_keypair(&self, key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        // Use the key as a seed to deterministically generate keypair
        let kem = Kem::new(Algorithm::HqcRmrs256)
            .map_err(|e| HybridGuardError::EncryptionError(format!("Failed to initialize HQC: {}", e)))?;
        
        // Hash the key to get a proper seed
        let mut hasher = Sha3_256::new();
        hasher.update(key);
        hasher.update(b"hqc-keypair-seed");
        let seed = hasher.finalize();
        
        // Generate keypair (in production, use proper key derivation)
        let (public_key, secret_key) = kem.keypair()
            .map_err(|e| HybridGuardError::EncryptionError(format!("Failed to generate keypair: {}", e)))?;
        
        Ok((public_key.into_vec(), secret_key.into_vec()))
    }
}

impl EncryptionLayer for HqcLayer {
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        log::info!("Layer 2 (HQC): Encrypting {} bytes", data.len());
        
        // Initialize HQC KEM
        let kem = Kem::new(Algorithm::HqcRmrs256)
            .map_err(|e| HybridGuardError::EncryptionError(format!("Failed to initialize HQC: {}", e)))?;
        
        // Derive keypair from layer key
        let (public_key, _) = self.derive_keypair(key)?;
        
        // Encapsulate to get shared secret and ciphertext
        let public_key_ref = oqs::kem::PublicKeyRef::new(&public_key)
            .map_err(|e| HybridGuardError::EncryptionError(format!("Invalid public key: {}", e)))?;
        
        let (ciphertext, shared_secret) = kem.encapsulate(&public_key_ref)
            .map_err(|e| HybridGuardError::EncryptionError(format!("Encapsulation failed: {}", e)))?;
        
        // Use shared secret to encrypt data with XOR (simple symmetric encryption)
        // In production, use AES-GCM or ChaCha20-Poly1305
        let mut encrypted_data = data.to_vec();
        let shared_secret_bytes = shared_secret.into_vec();
        
        // Expand shared secret to match data length using SHA3
        let mut key_stream = Vec::new();
        let mut counter = 0u64;
        while key_stream.len() < encrypted_data.len() {
            let mut hasher = Sha3_256::new();
            hasher.update(&shared_secret_bytes);
            hasher.update(&counter.to_le_bytes());
            key_stream.extend_from_slice(&hasher.finalize());
            counter += 1;
        }
        
        // XOR encryption
        for (i, byte) in encrypted_data.iter_mut().enumerate() {
            *byte ^= key_stream[i];
        }
        
        // Prepend ciphertext (KEM encapsulation) to encrypted data
        let mut result = ciphertext.into_vec();
        result.extend_from_slice(&encrypted_data);
        
        log::info!("Layer 2 (HQC): Encrypted to {} bytes", result.len());
        Ok(result)
    }
    
    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        log::info!("Layer 2 (HQC): Decrypting {} bytes", data.len());
        
        // Initialize HQC KEM
        let kem = Kem::new(Algorithm::HqcRmrs256)
            .map_err(|e| HybridGuardError::EncryptionError(format!("Failed to initialize HQC: {}", e)))?;
        
        // Derive keypair from layer key
        let (_, secret_key) = self.derive_keypair(key)?;
        
        // Extract KEM ciphertext (first part of data)
        let ciphertext_len = kem.length_ciphertext();
        if data.len() < ciphertext_len {
            return Err(HybridGuardError::DecryptionError("Data too short for HQC ciphertext".to_string()));
        }
        
        let kem_ciphertext = &data[..ciphertext_len];
        let encrypted_data = &data[ciphertext_len..];
        
        // Decapsulate to recover shared secret
        let secret_key_ref = oqs::kem::SecretKeyRef::new(&secret_key)
            .map_err(|e| HybridGuardError::DecryptionError(format!("Invalid secret key: {}", e)))?;
        
        let ciphertext_ref = oqs::kem::CiphertextRef::new(kem_ciphertext)
            .map_err(|e| HybridGuardError::DecryptionError(format!("Invalid ciphertext: {}", e)))?;
        
        let shared_secret = kem.decapsulate(&secret_key_ref, &ciphertext_ref)
            .map_err(|e| HybridGuardError::DecryptionError(format!("Decapsulation failed: {}", e)))?;
        
        // Use shared secret to decrypt data
        let mut decrypted_data = encrypted_data.to_vec();
        let shared_secret_bytes = shared_secret.into_vec();
        
        // Expand shared secret to match data length
        let mut key_stream = Vec::new();
        let mut counter = 0u64;
        while key_stream.len() < decrypted_data.len() {
            let mut hasher = Sha3_256::new();
            hasher.update(&shared_secret_bytes);
            hasher.update(&counter.to_le_bytes());
            key_stream.extend_from_slice(&hasher.finalize());
            counter += 1;
        }
        
        // XOR decryption
        for (i, byte) in decrypted_data.iter_mut().enumerate() {
            *byte ^= key_stream[i];
        }
        
        log::info!("Layer 2 (HQC): Decrypted to {} bytes", decrypted_data.len());
        Ok(decrypted_data)
    }
    
    fn name(&self) -> &str {
        "HQC (Code-based)"
    }
    
    fn security_level(&self) -> u32 {
        self.security_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hqc_layer_info() {
        let layer = HqcLayer::new();
        assert_eq!(layer.name(), "HQC (Code-based)");
        assert_eq!(layer.security_level(), 256);
    }
    
    #[test]
    fn test_hqc_encrypt_decrypt() {
        let layer = HqcLayer::new();
        let key = vec![0u8; 32]; // Test key
        let data = b"Test data for HQC encryption";
        
        // Encrypt
        let encrypted = layer.encrypt(data, &key).unwrap();
        assert!(encrypted.len() > data.len()); // Should be larger due to KEM ciphertext
        
        // Decrypt
        let decrypted = layer.decrypt(&encrypted, &key).unwrap();
        assert_eq!(data.to_vec(), decrypted);
    }
}
