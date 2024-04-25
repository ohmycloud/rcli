use clap::Parser;
use rcli::cli::text::TextSubCommand;
use rcli::cli::Base64SubCommand;
use rcli::process::{process_decode, process_encode};
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
                process_encode(opts.input, opts.format)?;
                Ok(())
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(opts.input, opts.format)?;
                Ok(())
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                println!("{:?}", opts);
                Ok(())
            }
            TextSubCommand::Verify(opts) => {
                println!("{:?}", opts);
                Ok(())
            }
        },
    }
}
