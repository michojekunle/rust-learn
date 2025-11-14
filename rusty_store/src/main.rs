mod cli;
mod store;

use cli::{Cli, Commands};
use clap::Parser;
use store::Store;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut store = Store::new(&cli.file)?;

    match cli.command {
        Commands::Set { key, value } => {
            store.set(key, value);
            store.save()?;
            println!("OK");
        }
        Commands::Get { key } => match store.get(&key) {
            Some(v) => println!("{}", v),
            None => {
                eprintln!("(not found)");
                std::process::exit(1);
            }
        },
        Commands::Delete { key } => {
            if store.delete(&key) {
                store.save()?;
                println!("Deleted.");
            } else {
                eprintln!("(not found)");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
