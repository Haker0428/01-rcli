use clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long, default_value = "output.json", value_parser = verify_input_file)]
    pub out: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not exists".into())
    }
}
