use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM 256-bit
use argon2::Argon2;
use dirs::home_dir;
use rand::{rngs::OsRng, RngCore};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

// handle errors
pub fn create_file() -> Result<PathBuf, Error> {
    let mut path: PathBuf = PathBuf::new();
    let pathbuffer = home_dir();
    match pathbuffer {
        Some(p) => {
            path.push(p);
            path.push(".secretstore");
            path.push("vault.txt");
        }
        None => {
            eprintln!("Could not determine home directory");
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            ));
        }
    }
    println!("{:?}", path);
    
    // Create the .secretstore directory if it doesn't exist
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Could not create directory: {}", e);
            return Err(e);
        }
    }
    
    let file = fs::OpenOptions::new().create(true).append(true).open(&path);
    match file {
        Ok(_) => Ok(path),
        Err(e) => {
            eprintln!("Could not create the file");
            Err(e)
        }
    }
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
