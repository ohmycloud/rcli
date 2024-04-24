use clap::Parser;
use rcli::cli::Base64SubCommand;
use rcli::{cli::Opts, cli::SubCommand, process::process_csv, process::process_genpass};

fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    match cli.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
            Ok(())
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.lowercase,
                opts.uppercase,
                opts.number,
                opts.symbol,
            );
            if let Ok(password) = password {
                println!("{}", password);
            }
            Ok(())
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                println!("{:?}", opts);
                Ok(())
            }
            Base64SubCommand::Decode(opts) => {
                println!("{:?}", opts);
                Ok(())
            }
        },
    }
}
