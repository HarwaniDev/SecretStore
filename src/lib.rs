use std::fs;

use dirs::home_dir;

// handle errors
pub fn create_file() {
    let mut path = home_dir().expect("Could not determine home directory");
    path.push(".secretstore");
    path.push("vault.txt");
    let _ = fs::OpenOptions::new().create(true).append(true).open(&path).expect("Could not create the file");
}



