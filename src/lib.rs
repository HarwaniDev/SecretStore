use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM 256-bit
use argon2::{
    password_hash::{PasswordHash, SaltString},
    Argon2, PasswordHasher,
};
use dirs::home_dir;
use rand::{rngs::OsRng, RngCore};
use std::fs;

// handle errors
pub fn create_file() {
    let mut path = home_dir().expect("Could not determine home directory");
    path.push(".secretstore");
    path.push("vault.txt");
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("Could not create the file");
}

fn derive_key(master_password: &str, salt: &[u8]) -> [u8; 32] {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .unwrap();
    key
}

pub fn encrypt_data(master_password: &str, plaintext: &[u8]) -> Vec<u8> {
    let mut salt = [0u8; 16];
    let mut rng = OsRng; // Create an OsRng instance

    rng.fill_bytes(&mut salt);

    let key = derive_key(master_password, &salt);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));

    let mut nonce = [0u8; 12];
    rng.fill_bytes(&mut nonce);

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .unwrap();

    // Store: [salt (16 bytes)] [nonce (12 bytes)] [ciphertext]
    [salt.to_vec(), nonce.to_vec(), ciphertext].concat()
}

pub fn decrypt_data(master_password: &str, data: &[u8]) -> Vec<u8> {
    let salt = &data[..16];
    let nonce = &data[16..28];
    let ciphertext = &data[28..];

    let key = derive_key(master_password, salt);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .unwrap()
}
