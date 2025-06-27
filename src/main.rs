use clap::{Parser, Subcommand};
use dirs::home_dir;
use rpassword::prompt_password;
use secretstore::{create_file, decrypt_data, encrypt_data};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(name = "ss")]
#[command(version= "1.0", about = "SecretStore - Your terminal-based password manager", long_about = None)]

struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // initialise secretstore
    Init,

    // add details for a platform
    Add {
        platform: String,
        username: String,
        password: String,
    },
    // list all the platforms
    List,

    // get a specfic platform details
    Get {
        platform: String,
    },

    // delete a specific platform details
    Delete {
        platform: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct AddInputs {
    platform: String,
    username: String,
    password: String,
}

fn main() {
    let cli = Cli::parse();
    let mut path = home_dir().expect("Could not determine home directory");
    path.push(".secretstore");
    path.push("vault.txt");
    
    match &cli.command {
        Some(commands) => match commands {
            Commands::Init => {
                let mut initialised = false;
                if path.exists() {
                    println!("SecretStore is already initialised at {:?}", path);
                    initialised = true;
                }
                if !initialised {
                    println!("Welcome to SecretStore 🔐\nYour personal, secure, and simple password manager — right in your terminal.\nYour privacy matters. Your secrets stay yours.");
                    if let Err(e) = create_file() {
                        eprintln!("Failed to create directory: {e}");
                        return;
                    }
                    let mut condition = true;
                    while condition {
                        let initial_password = match rpassword::prompt_password("Set your master password. You will have to use this password to get other passwords.\n Enter password: ") {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("Error reading password: {e}");
                                    continue;
                                }
                            };
                        let confirm_password =
                            match rpassword::prompt_password("Confirm your password: ") {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("Error reading password: {e}");
                                    continue;
                                }
                            };
                        if initial_password != confirm_password {
                            println!("Please enter same passwords.")
                        } else {
                            condition = false;
                        }
                    }
                }
            }

            Commands::Add {
                platform,
                username,
                password,
            } => {
                println!("Adding credentials for platform: {platform}");
                let entry = AddInputs {
                    platform: platform.to_string(),
                    username: username.to_string(),
                    password: password.to_string(),
                };
                let buffer = if path.exists() {
                    match fs::read(&path) {
                        Ok(buf) => buf,
                        Err(e) => {
                            eprintln!("Failed to read file: {e}");
                            Vec::new()
                        }
                    }
                } else {
                    Vec::new()
                };

                let (master_password, mut entries) = if !buffer.is_empty() {
                    loop {
                        let master_password = match prompt_password("Enter your master password: ")
                        {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("Error reading password: {e}");
                                continue;
                            }
                        };
                        let decrypted = match std::panic::catch_unwind(|| {
                            decrypt_data(&master_password, &buffer)
                        }) {
                            Ok(data) => data,
                            Err(_) => {
                                println!("Incorrect password. Please try again.");
                                continue;
                            }
                        };
                        let json = String::from_utf8(decrypted).unwrap_or_else(|_| String::new());
                        let entries: Vec<AddInputs> =
                            serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
                        break (master_password, entries);
                    }
                } else {
                    let master_password =
                        match prompt_password("Set your master password for this new vault: ") {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("Error reading password: {e}");
                                return;
                            }
                        };
                    (master_password, Vec::new())
                };
                entries.push(entry);
                let serialized = serde_json::to_string_pretty(&entries).unwrap();
                let encrypted_data = encrypt_data(&master_password, serialized.as_bytes());
                if let Err(e) = fs::write(&path, encrypted_data) {
                    eprintln!("Failed to write to file: {e}");
                }
            }

            Commands::List => {
                println!("Listing stored credentials...");
                let buffer = match fs::read(&path) {
                    Ok(buf) => buf,
                    Err(e) => {
                        eprintln!("Failed to read file: {e}");
                        return;
                    }
                };
                if buffer.is_empty() {
                    println!("No credentials stored.");
                    return;
                }
                let entries = loop {
                    let master_password = match prompt_password("Enter your master password: ") {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("Error reading password: {e}");
                            continue;
                        }
                    };
                    let decrypted_data = match std::panic::catch_unwind(|| {
                        decrypt_data(&master_password, &buffer)
                    }) {
                        Ok(data) => data,
                        Err(_) => {
                            println!("Incorrect password. Please try again.");
                            continue;
                        }
                    };
                    let json = String::from_utf8(decrypted_data).unwrap_or_else(|_| String::new());
                    let entries: Vec<AddInputs> =
                        serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
                    break entries;
                };
                for entry in entries {
                    println!(
                        "Platform: {} | Username: {}",
                        entry.platform, entry.username
                    );
                }
            }

            Commands::Get { platform } => {
                println!("Getting credentials for platform: {platform}");
                let buffer = match fs::read(&path) {
                    Ok(buf) => buf,
                    Err(e) => {
                        eprintln!("Failed to read file: {e}");
                        return;
                    }
                };
                if buffer.is_empty() {
                    println!("No credentials stored.");
                    return;
                }
                let result = loop {
                    let master_password = match prompt_password("Enter your master password: ") {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("Error reading password: {e}");
                            continue;
                        }
                    };
                    let decrypted_data = match std::panic::catch_unwind(|| {
                        decrypt_data(&master_password, &buffer)
                    }) {
                        Ok(data) => data,
                        Err(_) => {
                            println!("Incorrect password. Please try again.");
                            continue;
                        }
                    };
                    let json = String::from_utf8(decrypted_data).unwrap_or_else(|_| String::new());
                    let entries: Vec<AddInputs> =
                        serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
                    if let Some(entry) = entries
                        .into_iter()
                        .find(|entry| entry.platform == *platform)
                    {
                        break Some((entry.platform, entry.username, entry.password));
                    } else {
                        break None;
                    }
                };
                match result {
                    Some((platform, username, password)) => {
                        println!("Platform: {}", platform);
                        println!("Username: {}", username);
                        println!("Password: {}", password);
                    }
                    None => {
                        println!("No credentials found for platform: {platform}");
                    }
                }
            }

            Commands::Delete { platform } => {
                println!("Deleting credentials for platform: {platform}");
                let buffer = match fs::read(&path) {
                    Ok(buf) => buf,
                    Err(e) => {
                        eprintln!("Failed to read file: {e}");
                        return;
                    }
                };
                if buffer.is_empty() {
                    println!("No credentials stored.");
                    return;
                }
                let (master_password, mut entries) = loop {
                    let master_password = match prompt_password("Enter your master password: ") {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("Error reading password: {e}");
                            continue;
                        }
                    };
                    let decrypted_data = match std::panic::catch_unwind(|| {
                        decrypt_data(&master_password, &buffer)
                    }) {
                        Ok(data) => data,
                        Err(_) => {
                            println!("Incorrect password. Please try again.");
                            continue;
                        }
                    };
                    let json = String::from_utf8(decrypted_data).unwrap_or_else(|_| String::new());
                    let entries: Vec<AddInputs> =
                        serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
                    break (master_password, entries);
                };
                let initial_len = entries.len();
                entries.retain(|entry| entry.platform != *platform);
                if entries.len() == initial_len {
                    println!("No credentials found for platform: {platform}");
                } else {
                    println!("Credentials for platform '{platform}' deleted.");
                    let serialized = serde_json::to_string_pretty(&entries).unwrap();
                    let encrypted_data = encrypt_data(&master_password, serialized.as_bytes());
                    if let Err(e) = fs::write(&path, encrypted_data) {
                        eprintln!("Failed to write to file: {e}");
                    }
                }
            }
        },
        None => {
            println!("no arguements provided");
        }
    }
}
