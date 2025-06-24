use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ss")]
#[command(about = "SecretStore - Your terminal-based password manager", long_about = None)]

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
    
}
