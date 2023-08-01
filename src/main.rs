use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    print_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

fn print_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            #[allow(unused_must_use)]
            {
                writeln!(writer, "{}", line);
            }
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    print_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
