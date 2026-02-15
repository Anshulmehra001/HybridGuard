// Error handling for HybridGuard

use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum HybridGuardError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Decryption error: {0}")]
    Decryption(String),
    
    #[error("Key generation error: {0}")]
    KeyGeneration(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Layer error: {0}")]
    Layer(String),
}

pub type Result<T> = std::result::Result<T, HybridGuardError>;
