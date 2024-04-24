use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Decode(Base64Encoder),
    #[command(name = "decode", about = "Decode a base64 string")]
    Encode(Base64Decoder),
}

#[derive(Debug, Parser)]
pub struct Base64Encoder {
    #[arg(short, long)]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64Decoder {
    #[arg(short, long)]
    pub input: String,
}
