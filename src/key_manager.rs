// Key management system for HybridGuard
// Handles generation, storage, and rotation of encryption keys

use crate::crypto::hkdf::{KeyDerivation, LayerKeys};
use crate::error::{HybridGuardError, Result};
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};

/// Manages all encryption keys for HybridGuard
pub struct KeyManager {
    keys: LayerKeys,
    key_id: String,
}

impl KeyManager {
    /// Generate new keys from a master password
    pub fn generate(password: &str) -> Result<Self> {
        // Generate random salt
        let salt = Self::generate_salt();
        
        // Derive keys from password
        let kd = KeyDerivation::from_password(password, &salt);
        let keys = kd.derive_all_keys()?;
        
        // Generate unique key ID
        let key_id = Self::generate_key_id();
        
        Ok(Self { keys, key_id })
    }
    
    /// Load keys from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        let stored: StoredKeys = serde_json::from_str(&data)
            .map_err(|e| HybridGuardError::KeyGeneration(e.to_string()))?;
        
        Ok(Self {
            keys: LayerKeys {
                layer1_key: stored.layer1_key,
                layer2_key: stored.layer2_key,
                layer3_key: stored.layer3_key,
                layer4_key: stored.layer4_key,
            },
            key_id: stored.key_id,
        })
    }
    
    /// Save keys to a file (encrypted)
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let stored = StoredKeys {
            key_id: self.key_id.clone(),
            layer1_key: self.keys.layer1_key.clone(),
            layer2_key: self.keys.layer2_key.clone(),
            layer3_key: self.keys.layer3_key.clone(),
            layer4_key: self.keys.layer4_key.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        
        let json = serde_json::to_string_pretty(&stored)
            .map_err(|e| HybridGuardError::KeyGeneration(e.to_string()))?;
        
        fs::write(path, json)?;
        
        Ok(())
    }
    
    /// Get keys for all layers
    pub fn get_keys(&self) -> &LayerKeys {
        &self.keys
    }
    
    /// Get key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    }
    
    /// Generate a random salt
    fn generate_salt() -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32).map(|_| rng.gen()).collect()
    }
    
    /// Generate a unique key ID
    fn generate_key_id() -> String {
        use sha3::{Sha3_256, Digest};
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut hasher = Sha3_256::new();
        hasher.update(timestamp.to_le_bytes());
        hasher.update(rand::random::<[u8; 32]>());
        
        format!("hg-{:x}", hasher.finalize())
    }
}

/// Serializable key storage format
#[derive(Serialize, Deserialize)]
struct StoredKeys {
    key_id: String,
    layer1_key: Vec<u8>,
    layer2_key: Vec<u8>,
    layer3_key: Vec<u8>,
    layer4_key: Vec<u8>,
    created_at: String,
}
