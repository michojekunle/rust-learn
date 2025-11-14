use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Tiny file-based key-value store")]
pub struct Cli {
    /// Path to the JSON data file
    #[arg(short, long, default_value = "data.json")]
    pub file: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Set a key to a value
    Set {
        key: String,
        value: String,
    },
    /// Get the value for a key
    Get {
        key: String,
    },
    /// Delete a key
    Delete {
        key: String,
    },
}