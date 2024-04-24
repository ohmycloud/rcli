mod base64;
pub mod csv;
mod genpass;

pub use self::{base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat};
use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;

// 检查输入路径是否存在
pub fn verify_input_file(input_path: &str) -> Result<String, &'static str> {
    if input_path == "-" || std::path::Path::new(input_path).exists() {
        Ok(input_path.into())
    } else {
        Err("file doesn't exist")
    }
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("non-exist"), Err("file doesn't exist"));
    }
}
