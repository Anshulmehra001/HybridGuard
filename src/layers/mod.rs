// Encryption layers module
// Each layer provides independent quantum-resistant encryption

pub mod layer1_mlkem;
pub mod layer2_hqc;
pub mod layer3_noise;
pub mod layer4_fhe;

use crate::error::Result;

/// Trait that all encryption layers must implement
pub trait EncryptionLayer {
    /// Encrypt data using this layer
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>>;
    
    /// Decrypt data using this layer
    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>>;
    
    /// Get the name of this layer
    fn name(&self) -> &str;
    
    /// Get security level in bits
    fn security_level(&self) -> u32;
}
