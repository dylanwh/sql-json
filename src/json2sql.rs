use clap::Parser;
use eyre::Result;
use std::{io::Read, path::PathBuf};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about)]
struct Options {
    /// File to parse
    #[arg(default_value = "-")]
    file: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut buffer = String::new();

    if options.file == PathBuf::from("-") {
        std::io::stdin().read_to_string(&mut buffer)?;
    } else {
        buffer = std::fs::read_to_string(&options.file)?;
    }

    let ast: Vec<sqlparser::ast::Statement> = serde_json::from_str(&buffer)?;
    for statement in ast {
        println!("{statement};");
    }

    Ok(())
}
