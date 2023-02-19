use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;

/// `Cli` is a struct with two fields, `pattern` and `path`, where `pattern` is a `String` and `path` is
/// a `std::path::PathBuf`.
///
/// The `#[derive(Parser)]` attribute tells the Rust compiler to generate a parser for the `Cli` type.
///
/// The `pattern` field is annotated with `#[opt]`, which tells the parser to treat it as an optional
/// argument.
///
/// The `path` field is annotated with `#[pos]`, which tells the
///
/// Properties:
///
/// * `pattern`: The pattern to search for.
/// * `path`: The path to the file to search.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

/// It opens a file, reads its contents, and then prints out all the lines that contain a given pattern
///
/// Returns:
///
/// Result<()>
fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", &args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
