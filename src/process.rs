use crate::{OutputFormat, ParcelOrder};
use anyhow::Error;
use csv::Reader;
use serde_json::Value;
use std::fs;

pub fn process_csv_to_json(input: &str, output: &str, format: OutputFormat) -> Result<(), Error> {
    let mut reader = Reader::from_path(input)?;

    let headers = reader.headers()?.clone();

    // println!("{:?}", headers);

    let records: Vec<_> = reader
        .records()
        .map(|record| {
            let mut r = record.unwrap();
            let cv: Value = headers.iter().zip(r.iter()).collect();
            cv
        })
        .collect();

    println!("{:?}", records);

    // let json = serde_json::to_string_pretty(&records)?;
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
    };
    fs::write(output, content)?;
    Ok(())
}
