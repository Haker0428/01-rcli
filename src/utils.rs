use anyhow::Result;
use std::{fs::File, io::Read};

#[allow(dead_code)]
pub fn get_file_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
