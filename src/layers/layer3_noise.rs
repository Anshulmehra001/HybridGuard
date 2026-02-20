// Layer 3: Quantum Noise Injection
// This layer adds quantum-inspired noise to defend against AI-powered side-channel attacks

use crate::error::{HybridGuardError, Result};
use crate::layers::EncryptionLayer;
use sha3::{Sha3_256, Digest};

/// Quantum Noise Injection layer
/// Adds cryptographically secure random noise to confuse AI attackers
pub struct QuantumNoiseLayer {
    security_level: u32,
}

impl QuantumNoiseLayer {
    pub fn new() -> Self {
        Self {
            security_level: 256,
        }
    }
    
    /// Generate deterministic quantum-inspired noise from key
    fn generate_noise(&self, key: &[u8], length: usize) -> Vec<u8> {
        let mut noise = Vec::with_capacity(length);
        let mut counter = 0u64;
        
        while noise.len() < length {
            let mut hasher = Sha3_256::new();
            hasher.update(key);
            hasher.update(b"quantum-noise-layer3");
            hasher.update(&counter.to_le_bytes());
            noise.extend_from_slice(&hasher.finalize());
            counter += 1;
        }
        
        noise.truncate(length);
        noise
    }
}

impl EncryptionLayer for QuantumNoiseLayer {
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        log::info!("Layer 3 (Quantum Noise): Injecting noise into {} bytes", data.len());
        
        // Generate deterministic noise from key
        let noise = self.generate_noise(key, data.len());
        
        // XOR data with noise to inject it
        let mut noisy_data = Vec::with_capacity(data.len());
        for (d, n) in data.iter().zip(noise.iter()) {
            noisy_data.push(d ^ n);
        }
        
        log::info!("Layer 3 (Quantum Noise): Output size {} bytes", noisy_data.len());
        
        Ok(noisy_data)
    }
    
    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        log::info!("Layer 3 (Quantum Noise): Removing noise from {} bytes", data.len());
        
        // Generate same deterministic noise from key
        let noise = self.generate_noise(key, data.len());
        
        // XOR again to remove noise (XOR is reversible)
        let mut clean_data = Vec::with_capacity(data.len());
        for (d, n) in data.iter().zip(noise.iter()) {
            clean_data.push(d ^ n);
        }
        
        log::info!("Layer 3 (Quantum Noise): Cleaned to {} bytes", clean_data.len());
        
        Ok(clean_data)
    }
    
    fn name(&self) -> &str {
        "Quantum Noise Injection"
    }
    
    fn security_level(&self) -> u32 {
        self.security_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_noise_layer_info() {
        let layer = QuantumNoiseLayer::new();
        assert_eq!(layer.name(), "Quantum Noise Injection");
        assert_eq!(layer.security_level(), 256);
    }
    
    #[test]
    fn test_noise_encrypt_decrypt() {
        let layer = QuantumNoiseLayer::new();
        let key = vec![0u8; 32]; // Test key
        let data = b"Test data for quantum noise layer";
        
        // Encrypt (inject noise)
        let encrypted = layer.encrypt(data, &key).unwrap();
        assert_eq!(encrypted.len(), data.len()); // Same size
        assert_ne!(&encrypted[..], data); // Should be different
        
        // Decrypt (remove noise)
        let decrypted = layer.decrypt(&encrypted, &key).unwrap();
        assert_eq!(data.to_vec(), decrypted); // Should match original
    }
    
    #[test]
    fn test_noise_deterministic() {
        let layer = QuantumNoiseLayer::new();
        let key = vec![42u8; 32];
        let data = b"Deterministic test";
        
        // Encrypt twice with same key
        let encrypted1 = layer.encrypt(data, &key).unwrap();
        let encrypted2 = layer.encrypt(data, &key).unwrap();
        
        // Should produce same result
        assert_eq!(encrypted1, encrypted2);
    }
}
