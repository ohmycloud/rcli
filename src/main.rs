use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    match cli.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
            Ok(())
        }
    }
}
