use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

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

                    let mut file = fs::OpenOptions::new()
                        .append(true)
                        .open(&path)
                        .expect("Failed to open file in append mode");

                    let entry = AddInputs {
                        platform: platform.to_string(),
                        username: username.to_string(),
                        password: password.to_string()
                    };

                    // serialize entry using serde
                    let serialized = serde_json::to_string(&entry).unwrap();
                    file.write_all(serialized.as_bytes())
                        .expect("Failed to write to file");
                }

                Commands::List => {
                    println!("Listing stored credentials...");
                    let contents = fs::read_to_string(&path).unwrap();
                    let deserialized: AddInputs = serde_json::from_str(&contents).unwrap();
                    println!("{:?}", deserialized);
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
