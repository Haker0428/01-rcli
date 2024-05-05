mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::base64::Base64Format;
pub use self::base64::Base64SubCommand;
pub use self::csv::OutputFormat;
use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap[name  = "rcli", version, author, about, long_about = None]]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate the password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not exists")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exists"), Err("File not exists"));
    }
}
