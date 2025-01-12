use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// the patternt o look for
    pattern: String,
    /// the path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

