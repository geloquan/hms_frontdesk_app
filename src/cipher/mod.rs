use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
use rand::Rng;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, BufRead, Read};
use std::path::Path;

const PASSPHRASE: &str = "yeeey";

pub enum Error {
    
}

#[derive(Deserialize, Debug, Serialize)]
pub struct EncryptedText {
    pub nonce: Vec<u8>,
    pub cipher_text: Vec<u8>
}
pub fn generate_fixed_key() -> Result<Key<Aes256Gcm>, &'static str> {
    let mut hasher = Sha256::new();
    
    if let Ok(file) = fs::read_to_string("key.txt") {
        hasher.update(file);
        let result = hasher.finalize();
        let key_bytes: [u8; 32] = result.as_slice().try_into().expect("SHA-256 must produce 32 bytes");
        
        Ok(Key::<Aes256Gcm>::from_slice(&key_bytes).clone())
    } else {
        println!("err key.txt");
        Err("err")
    }
}

pub fn encrypt_message(key: &Key<Aes256Gcm>, message: &str) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new(key);
    
    // Generate a random 12-byte nonce (IV)
    let binding = rand::thread_rng().gen::<[u8; 12]>();
    let nonce = Nonce::from_slice(&binding);
    
    // Encrypt the message
    let ciphertext = cipher.encrypt(nonce, message.as_bytes()).expect("encryption failed");
    
    (nonce.to_vec(), ciphertext) // Return the nonce and ciphertext
}

pub fn decrypt_message(key: &Key<Aes256Gcm>, nonce: &[u8], ciphertext: &[u8]) -> Result<String, &'static str> {
    let cipher = Aes256Gcm::new(key);
    if let Ok(decrypted) = cipher.decrypt(Nonce::from_slice(nonce), ciphertext) {
        if let Ok(decrypted) = String::from_utf8(decrypted) {
            Ok(decrypted)
        } else {
            Err("err")
        }
    } else {
        Err("err")
    }
}