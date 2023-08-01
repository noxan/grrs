use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let result = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()));

    let content = match result {
        Ok(content) => content,
        Err(error) => return Err(error.into()),
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
