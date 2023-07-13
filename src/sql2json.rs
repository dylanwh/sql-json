use clap::Parser;
use eyre::Result;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser;
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

    let dialect = MySqlDialect {};
    let ast = parser::Parser::parse_sql(&dialect, &buffer)?;
    println!("{}", serde_json::to_string_pretty(&ast)?);

    Ok(())
}
