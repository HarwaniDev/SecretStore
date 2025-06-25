use serde::{Deserialize, Serialize};
use std::fs;

use clap::{Parser, Subcommand};
use dirs::home_dir;
use secretstore::create_file;

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
        Some(commands) => {
            match commands {
                Commands::Init => {
                    println!("Initializing SecretStore...");
                    create_file();
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

                    // Read existing entries or start fresh
                    let mut entries: Vec<AddInputs> = if path.exists() {
                        let contents = fs::read_to_string(&path).unwrap();
                        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
                    } else {
                        Vec::new()
                    };

                    // Add new entry
                    entries.push(entry);

                    // Serialize the whole array back to JSON
                    let serialized = serde_json::to_string_pretty(&entries).unwrap();

                    // Overwrite the file with updated array
                    fs::write(&path, serialized).expect("Failed to write to file");
                }

                Commands::List => {
                    println!("Listing stored credentials...");

                    let contents = fs::read_to_string(&path).unwrap();
                    let entries: Vec<AddInputs> = serde_json::from_str(&contents).unwrap();

                    for entry in entries {
                        println!("{:?}", entry);
                    }
                }

                Commands::Get { platform } => {
                    println!("Getting credentials for platform: {platform}");
                    // Your get logic here
                }

                Commands::Delete { platform } => {
                    println!("Deleting credentials for platform: {platform}");
                    // Your delete logic here
                }
            }
        }
        None => {
            println!("no arguements provided");
        }
    }
}
