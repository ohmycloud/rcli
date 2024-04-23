use clap::Parser;
use rcli::{process::process_csv, process::process_genpass, Opts, SubCommand};

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
    }
}
