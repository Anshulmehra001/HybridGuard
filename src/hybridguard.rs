// HybridGuard Core - Complete 4-layer encryption system

use crate::error::{HybridGuardError, Result};
use crate::key_manager::KeyManager;
use crate::layers::{EncryptionLayer, layer1_mlkem::MlKemLayer, layer2_hqc::HqcLayer, layer3_noise::QuantumNoiseLayer, layer4_fhe::FHELayer};
use crate::crypto::EncryptedData;
use std::time::Instant;

/// Main HybridGuard encryption system
/// Coordinates all 4 layers of encryption
pub struct HybridGuard {
    key_manager: KeyManager,
    layer1: MlKemLayer,
    layer2: HqcLayer,
    layer3: QuantumNoiseLayer,
    layer4: FHELayer,
}

impl HybridGuard {
    /// Create a new HybridGuard instance with a password
    pub fn new(password: &str) -> Result<Self> {
        let key_manager = KeyManager::generate(password)?;
        
        Ok(Self {
            key_manager,
            layer1: MlKemLayer::new(),
            layer2: HqcLayer::new(),
            layer3: QuantumNoiseLayer::new(),
            layer4: FHELayer::new(),
        })
    }
    
    /// Load HybridGuard with existing keys
    pub fn load(key_path: &str) -> Result<Self> {
        let key_manager = KeyManager::load(key_path)?;
        
        Ok(Self {
            key_manager,
            layer1: MlKemLayer::new(),
            layer2: HqcLayer::new(),
            layer3: QuantumNoiseLayer::new(),
            layer4: FHELayer::new(),
        })
    }
    
    /// Encrypt data through all 4 layers
    pub fn encrypt(&self, data: &[u8]) -> Result<EncryptedData> {
        let start = Instant::now();
        
        log::info!("Starting 4-layer encryption of {} bytes", data.len());
        
        let keys = self.key_manager.get_keys();
        
        // Layer 1: ML-KEM (Lattice-based)
        log::info!("🔐 Layer 1: ML-KEM encryption...");
        let layer1_data = self.layer1.encrypt(data, &keys.layer1_key)?;
        log::info!("   Output: {} bytes", layer1_data.len());
        
        // Layer 2: HQC (Code-based)
        log::info!("🔐 Layer 2: HQC encryption...");
        let layer2_data = self.layer2.encrypt(&layer1_data, &keys.layer2_key)?;
        log::info!("   Output: {} bytes", layer2_data.len());
        
        // Layer 3: Quantum Noise Injection
        log::info!("🔐 Layer 3: Quantum noise injection...");
        let layer3_data = self.layer3.encrypt(&layer2_data, &keys.layer3_key)?;
        log::info!("   Output: {} bytes", layer3_data.len());
        
        // Layer 4: Homomorphic Encryption
        log::info!("🔐 Layer 4: Homomorphic encryption...");
        let final_data = self.layer4.encrypt(&layer3_data, &keys.layer4_key)?;
        log::info!("   Output: {} bytes", final_data.len());
        
        let elapsed = start.elapsed();
        log::info!("✅ Encryption complete in {:?}", elapsed);
        
        Ok(EncryptedData::new(final_data))
    }
    
    /// Decrypt data through all 4 layers (in reverse)
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        let start = Instant::now();
        
        log::info!("Starting 4-layer decryption of {} bytes", encrypted.ciphertext.len());
        
        let keys = self.key_manager.get_keys();
        
        // Layer 4: Homomorphic Decryption
        log::info!("🔓 Layer 4: Homomorphic decryption...");
        let layer4_data = self.layer4.decrypt(&encrypted.ciphertext, &keys.layer4_key)?;
        log::info!("   Output: {} bytes", layer4_data.len());
        
        // Layer 3: Quantum Noise Removal
        log::info!("🔓 Layer 3: Quantum noise removal...");
        let layer3_data = self.layer3.decrypt(&layer4_data, &keys.layer3_key)?;
        log::info!("   Output: {} bytes", layer3_data.len());
        
        // Layer 2: HQC Decryption
        log::info!("🔓 Layer 2: HQC decryption...");
        let layer2_data = self.layer2.decrypt(&layer3_data, &keys.layer2_key)?;
        log::info!("   Output: {} bytes", layer2_data.len());
        
        // Layer 1: ML-KEM Decryption
        log::info!("🔓 Layer 1: ML-KEM decryption...");
        let plaintext = self.layer1.decrypt(&layer2_data, &keys.layer1_key)?;
        log::info!("   Output: {} bytes", plaintext.len());
        
        let elapsed = start.elapsed();
        log::info!("✅ Decryption complete in {:?}", elapsed);
        
        Ok(plaintext)
    }
    
    /// Get encryption statistics
    pub fn get_stats(&self) -> EncryptionStats {
        EncryptionStats {
            layers: vec![
                LayerInfo {
                    name: self.layer1.name().to_string(),
                    security_bits: self.layer1.security_level(),
                    status: "Active".to_string(),
                },
                LayerInfo {
                    name: self.layer2.name().to_string(),
                    security_bits: self.layer2.security_level(),
                    status: "Active".to_string(),
                },
                LayerInfo {
                    name: self.layer3.name().to_string(),
                    security_bits: self.layer3.security_level(),
                    status: "Active".to_string(),
                },
                LayerInfo {
                    name: self.layer4.name().to_string(),
                    security_bits: self.layer4.security_level(),
                    status: "Active".to_string(),
                },
            ],
            key_id: self.key_manager.key_id().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct EncryptionStats {
    pub layers: Vec<LayerInfo>,
    pub key_id: String,
}

#[derive(Debug)]
pub struct LayerInfo {
    pub name: String,
    pub security_bits: u32,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let hg = HybridGuard::new("test_password_123").unwrap();
        
        let plaintext = b"Hello, HybridGuard!";
        let encrypted = hg.encrypt(plaintext).unwrap();
        let decrypted = hg.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }
}
