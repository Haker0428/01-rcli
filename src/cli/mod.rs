mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::Path;
use std::path::PathBuf;

pub use self::base64::Base64Format;
pub use self::base64::Base64SubCommand;
pub use self::csv::OutputFormat;
pub use self::http::HttpSubcommand;
pub use self::text::TextSignFormat;
pub use self::text::TextSubcommand;
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
    #[command(subcommand)]
    Text(TextSubcommand),
    #[command(subcommand)]
    Http(HttpSubcommand),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not exists")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exists")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exists"), Err("File not exists"));
    }
}
