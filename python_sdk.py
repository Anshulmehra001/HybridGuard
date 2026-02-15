#!/usr/bin/env python3
"""
HybridGuard Python SDK
Simple wrapper around the HybridGuard CLI
"""

import subprocess
import os
from pathlib import Path
from typing import Optional


class HybridGuard:
    """HybridGuard encryption client"""
    
    def __init__(self, binary_path: str = "./target/release/hybridguard"):
        """
        Initialize HybridGuard client
        
        Args:
            binary_path: Path to hybridguard binary
        """
        self.binary_path = binary_path
        if not os.path.exists(binary_path):
            raise FileNotFoundError(f"HybridGuard binary not found: {binary_path}")
    
    def encrypt_file(self, input_path: str, output_path: str, key_path: Optional[str] = None) -> bool:
        """
        Encrypt a file
        
        Args:
            input_path: Path to input file
            output_path: Path to output encrypted file
            key_path: Optional path to key file
            
        Returns:
            True if successful, False otherwise
        """
        cmd = [self.binary_path, "encrypt", "-i", input_path, "-o", output_path]
        if key_path:
            cmd.extend(["-k", key_path])
        
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            return True
        except subprocess.CalledProcessError as e:
            print(f"Encryption failed: {e.stderr}")
            return False
    
    def decrypt_file(self, input_path: str, output_path: str, key_path: Optional[str] = None) -> bool:
        """
        Decrypt a file
        
        Args:
            input_path: Path to encrypted file
            output_path: Path to output decrypted file
            key_path: Optional path to key file
            
        Returns:
            True if successful, False otherwise
        """
        cmd = [self.binary_path, "decrypt", "-i", input_path, "-o", output_path]
        if key_path:
            cmd.extend(["-k", key_path])
        
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            return True
        except subprocess.CalledProcessError as e:
            print(f"Decryption failed: {e.stderr}")
            return False
    
    def generate_key(self, output_path: str) -> bool:
        """
        Generate a new encryption key
        
        Args:
            output_path: Path to save the key
            
        Returns:
            True if successful, False otherwise
        """
        cmd = [self.binary_path, "keygen", "-o", output_path]
        
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            return True
        except subprocess.CalledProcessError as e:
            print(f"Key generation failed: {e.stderr}")
            return False
    
    def get_status(self) -> str:
        """
        Get HybridGuard status
        
        Returns:
            Status string
        """
        cmd = [self.binary_path, "status"]
        
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            return result.stdout
        except subprocess.CalledProcessError as e:
            return f"Status check failed: {e.stderr}"


# Example usage
if __name__ == "__main__":
    # Initialize client
    hg = HybridGuard()
    
    # Get status
    print("Status:")
    print(hg.get_status())
    
    # Example: Encrypt a file
    # hg.encrypt_file("secret.txt", "secret.enc")
    
    # Example: Decrypt a file
    # hg.decrypt_file("secret.enc", "decrypted.txt")
    
    # Example: Generate key
    # hg.generate_key("mykey.json")
