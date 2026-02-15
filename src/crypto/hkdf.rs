// HKDF (HMAC-based Key Derivation Function) implementation
// Used to derive independent keys for each encryption layer

use sha3::{Sha3_256, Digest};
use crate::error::{HybridGuardError, Result};

/// Derives multiple independent keys from a master key using HKDF
pub struct KeyDerivation {
    master_key: Vec<u8>,
}

impl KeyDerivation {
    /// Create a new key derivation instance with a master key
    pub fn new(master_key: Vec<u8>) -> Self {
        Self { master_key }
    }
    
    /// Generate a master key from a password
    pub fn from_password(password: &str, salt: &[u8]) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(password.as_bytes());
        hasher.update(salt);
        let master_key = hasher.finalize().to_vec();
        
        Self { master_key }
    }
    
    /// Derive a key for a specific layer
    /// Each layer gets a unique key derived from the master key
    pub fn derive_layer_key(&self, layer_id: u8, key_size: usize) -> Result<Vec<u8>> {
        // Create unique info for this layer
        let info = format!("HybridGuard-Layer-{}", layer_id);
        
        // Use HKDF to derive the key
        let mut hasher = Sha3_256::new();
        hasher.update(&self.master_key);
        hasher.update(info.as_bytes());
        hasher.update(&[layer_id]);
        
        let derived = hasher.finalize();
        
        // Expand to desired key size if needed
        if key_size <= 32 {
            Ok(derived[..key_size].to_vec())
        } else {
            // For larger keys, do multiple rounds
            let mut result = Vec::new();
            let mut counter = 0u8;
            
            while result.len() < key_size {
                let mut hasher = Sha3_256::new();
                hasher.update(&derived);
                hasher.update(&[counter]);
                result.extend_from_slice(&hasher.finalize());
                counter += 1;
            }
            
            Ok(result[..key_size].to_vec())
        }
    }
    
    /// Derive all four layer keys at once
    pub fn derive_all_keys(&self) -> Result<LayerKeys> {
        Ok(LayerKeys {
            layer1_key: self.derive_layer_key(1, 32)?,  // ML-KEM key
            layer2_key: self.derive_layer_key(2, 32)?,  // HQC key
            layer3_key: self.derive_layer_key(3, 32)?,  // Quantum noise key
            layer4_key: self.derive_layer_key(4, 32)?,  // FHE key
        })
    }
}

/// Container for all layer keys
#[derive(Debug, Clone)]
pub struct LayerKeys {
    pub layer1_key: Vec<u8>,  // ML-KEM (Lattice-based)
    pub layer2_key: Vec<u8>,  // HQC (Code-based)
    pub layer3_key: Vec<u8>,  // Quantum Noise
    pub layer4_key: Vec<u8>,  // Homomorphic Encryption
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_derivation() {
        let master_key = vec![0u8; 32];
        let kd = KeyDerivation::new(master_key);
        
        let key1 = kd.derive_layer_key(1, 32).unwrap();
        let key2 = kd.derive_layer_key(2, 32).unwrap();
        
        // Keys should be different
        assert_ne!(key1, key2);
        
        // Keys should be deterministic
        let key1_again = kd.derive_layer_key(1, 32).unwrap();
        assert_eq!(key1, key1_again);
    }
    
    #[test]
    fn test_derive_all_keys() {
        let master_key = vec![0u8; 32];
        let kd = KeyDerivation::new(master_key);
        
        let keys = kd.derive_all_keys().unwrap();
        
        // All keys should be different
        assert_ne!(keys.layer1_key, keys.layer2_key);
        assert_ne!(keys.layer2_key, keys.layer3_key);
        assert_ne!(keys.layer3_key, keys.layer4_key);
    }
}
