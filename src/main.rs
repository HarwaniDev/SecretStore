use clap::{Parser, Subcommand};

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
        password: String
    },
    // list all the platforms
    List,

    // get a specfic platform details
    Get {
        platform: String
    },

    // delete a specific platform details
    Delete {
        platform: String
    }
}



fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(commands) => {
            match commands {
                Commands::Init => {
                    println!("Initializing SecretStore...");
                    // Your init logic here
                }
                Commands::Add { platform, username, password } => {
                    println!("Adding credentials for platform: {platform}");
                    // Your add logic here
                }
                Commands::List => {
                    println!("Listing stored credentials...");
                    // Your list logic here
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
