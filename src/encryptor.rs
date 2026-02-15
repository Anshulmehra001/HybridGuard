// Main encryption engine that orchestrates all 4 layers

use crate::crypto::EncryptedData;
use crate::crypto::hkdf::LayerKeys;
use crate::error::{HybridGuardError, Result};
use crate::layers::{
    EncryptionLayer,
    layer1_mlkem::MlKemLayer,
    layer2_hqc::HqcLayer,
    layer3_noise::QuantumNoiseLayer,
};
use std::time::Instant;

/// Main encryption engine that coordinates all 4 layers
pub struct HybridGuardEncryptor {
    layer1: MlKemLayer,
    layer2: HqcLayer,
    layer3: QuantumNoiseLayer,
    // layer4: FheLayer, // TODO: Add when FHE is implemented
}

impl HybridGuardEncryptor {
    /// Create a new encryptor with all layers
    pub fn new() -> Self {
        Self {
            layer1: MlKemLayer::new(),
            layer2: HqcLayer::new(),
            layer3: QuantumNoiseLayer::new(),
        }
    }
    
    /// Encrypt data through all 4 layers
    pub fn encrypt(&self, data: &[u8], keys: &LayerKeys) -> Result<EncryptedData> {
        let start = Instant::now();
        
        log::info!("Starting 4-layer encryption of {} bytes", data.len());
        
        // Layer 1: ML-KEM (Lattice-based)
        log::info!("🔐 Layer 1: ML-KEM encryption...");
        let layer1_output = self.layer1.encrypt(data, &keys.layer1_key)?;
        log::info!("   Output: {} bytes", layer1_output.len());
        
        // Layer 2: HQC (Code-based)
        log::info!("🔐 Layer 2: HQC encryption...");
        let layer2_output = self.layer2.encrypt(&layer1_output, &keys.layer2_key)?;
        log::info!("   Output: {} bytes", layer2_output.len());
        
        // Layer 3: Quantum Noise Injection
        log::info!("🔐 Layer 3: Quantum noise injection...");
        let layer3_output = self.layer3.encrypt(&layer2_output, &keys.layer3_key)?;
        log::info!("   Output: {} bytes", layer3_output.len());
        
        // Layer 4: Homomorphic Encryption (TODO)
        log::info!("🔐 Layer 4: Homomorphic encryption (coming soon)...");
        let final_output = layer3_output; // For now, skip layer 4
        
        let elapsed = start.elapsed();
        log::info!("✅ Encryption complete in {:?}", elapsed);
        log::info!("   Original size: {} bytes", data.len());
        log::info!("   Encrypted size: {} bytes", final_output.len());
        log::info!("   Expansion ratio: {:.2}x", final_output.len() as f64 / data.len() as f64);
        
        Ok(EncryptedData::new(final_output))
    }
    
    /// Decrypt data through all 4 layers (in reverse order)
    pub fn decrypt(&self, encrypted: &EncryptedData, keys: &LayerKeys) -> Result<Vec<u8>> {
        let start = Instant::now();
        
        log::info!("Starting 4-layer decryption of {} bytes", encrypted.ciphertext.len());
        
        // Layer 4: Homomorphic Decryption (TODO)
        log::info!("🔓 Layer 4: Homomorphic decryption (coming soon)...");
        let layer4_output = encrypted.ciphertext.clone(); // For now, skip layer 4
        
        // Layer 3: Quantum Noise Removal
        log::info!("🔓 Layer 3: Quantum noise removal...");
        let layer3_output = self.layer3.decrypt(&layer4_output, &keys.layer3_key)?;
        log::info!("   Output: {} bytes", layer3_output.len());
        
        // Layer 2: HQC Decryption
        log::info!("🔓 Layer 2: HQC decryption...");
        let layer2_output = self.layer2.decrypt(&layer3_output, &keys.layer2_key)?;
        log::info!("   Output: {} bytes", layer2_output.len());
        
        // Layer 1: ML-KEM Decryption
        log::info!("🔓 Layer 1: ML-KEM decryption...");
        let plaintext = self.layer1.decrypt(&layer2_output, &keys.layer1_key)?;
        log::info!("   Output: {} bytes", plaintext.len());
        
        let elapsed = start.elapsed();
        log::info!("✅ Decryption complete in {:?}", elapsed);
        
        Ok(plaintext)
    }
    
    /// Get information about all layers
    pub fn layer_info(&self) -> Vec<LayerInfo> {
        vec![
            LayerInfo {
                name: self.layer1.name().to_string(),
                security_level: self.layer1.security_level(),
                status: "Active".to_string(),
            },
            LayerInfo {
                name: self.layer2.name().to_string(),
                security_level: self.layer2.security_level(),
                status: "Active".to_string(),
            },
            LayerInfo {
                name: self.layer3.name().to_string(),
                security_level: self.layer3.security_level(),
                status: "Active".to_string(),
            },
            LayerInfo {
                name: "Homomorphic Encryption".to_string(),
                security_level: 256,
                status: "Coming Soon".to_string(),
            },
        ]
    }
}

/// Information about an encryption layer
#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub name: String,
    pub security_level: u32,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hkdf::KeyDerivation;
    
    #[test]
    fn test_encrypt_decrypt() {
        let encryptor = HybridGuardEncryptor::new();
        
        // Generate keys
        let kd = KeyDerivation::new(vec![0u8; 32]);
        let keys = kd.derive_all_keys().unwrap();
        
        // Test data
        let data = b"Hello, Quantum World!";
        
        // Encrypt
        let encrypted = encryptor.encrypt(data, &keys).unwrap();
        
        // Decrypt
        let decrypted = encryptor.decrypt(&encrypted, &keys).unwrap();
        
        // Verify
        assert_eq!(data.to_vec(), decrypted);
    }
    
    #[test]
    fn test_layer_info() {
        let encryptor = HybridGuardEncryptor::new();
        let info = encryptor.layer_info();
        
        assert_eq!(info.len(), 4);
        assert_eq!(info[0].name, "ML-KEM-768 (Lattice-based)");
        assert_eq!(info[1].name, "HQC (Code-based)");
        assert_eq!(info[2].name, "Quantum Noise Injection");
    }
}
