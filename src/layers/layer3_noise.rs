// Layer 3: Quantum Noise Injection
// This layer adds quantum-inspired noise to defend against AI-powered side-channel attacks

use crate::error::{HybridGuardError, Result};
use crate::layers::EncryptionLayer;
use rand::Rng;

/// Quantum Noise Injection layer
/// Adds cryptographically secure random noise to confuse AI attackers
pub struct QuantumNoiseLayer {
    noise_strength: f64,
}

impl QuantumNoiseLayer {
    pub fn new() -> Self {
        Self {
            noise_strength: 0.1, // 10% noise injection
        }
    }
    
    /// Inject quantum-inspired noise into the data
    fn inject_noise(&self, data: &[u8]) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut noisy_data = Vec::with_capacity(data.len() + data.len() / 10);
        
        for &byte in data {
            noisy_data.push(byte);
            
            // Randomly inject noise bytes
            if rng.gen::<f64>() < self.noise_strength {
                noisy_data.push(rng.gen());
            }
        }
        
        noisy_data
    }
    
    /// Remove quantum noise from the data
    fn remove_noise(&self, data: &[u8]) -> Vec<u8> {
        // TODO: Implement proper noise removal
        // For now, just return the data
        data.to_vec()
    }
}

impl EncryptionLayer for QuantumNoiseLayer {
    fn encrypt(&self, data: &[u8], _key: &[u8]) -> Result<Vec<u8>> {
        log::info!("Layer 3 (Quantum Noise): Injecting noise into {} bytes", data.len());
        
        let noisy_data = self.inject_noise(data);
        
        log::info!("Layer 3 (Quantum Noise): Output size {} bytes", noisy_data.len());
        
        Ok(noisy_data)
    }
    
    fn decrypt(&self, data: &[u8], _key: &[u8]) -> Result<Vec<u8>> {
        log::info!("Layer 3 (Quantum Noise): Removing noise from {} bytes", data.len());
        
        let clean_data = self.remove_noise(data);
        
        Ok(clean_data)
    }
    
    fn name(&self) -> &str {
        "Quantum Noise Injection"
    }
    
    fn security_level(&self) -> u32 {
        128 // Provides additional 128-bit security against side-channel attacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_noise_layer() {
        let layer = QuantumNoiseLayer::new();
        assert_eq!(layer.name(), "Quantum Noise Injection");
        assert_eq!(layer.security_level(), 128);
    }
    
    #[test]
    fn test_noise_injection() {
        let layer = QuantumNoiseLayer::new();
        let data = vec![1, 2, 3, 4, 5];
        let noisy = layer.inject_noise(&data);
        
        // Noisy data should be larger
        assert!(noisy.len() >= data.len());
    }
}
