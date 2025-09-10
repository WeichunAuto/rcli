use anyhow;
use clap::Parser;
use rcli::{Opts, SubCommand, process_csv_to_json};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv_to_json(&opts.input, opts.output.as_str())?;
        }
    }
    Ok(())
}
