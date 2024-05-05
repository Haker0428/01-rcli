mod cli;
mod process;

pub use cli::{Base64Format, Base64SubCommand, Opts, Subcommand};
pub use process::{process_csv, process_genpass};
pub use process::{process_decode, process_encode};
