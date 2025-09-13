use anyhow;
use clap::Parser;
use rcli::{Opts, SubCommand, process_csv_to_json};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // "output.json".into()
                format!("output.{:?}", opts.format)
            };
            process_csv_to_json(&opts.input, &output, opts.format)?;
        }
    }
    Ok(())
}
