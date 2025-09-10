use crate::ParcelOrder;
use anyhow::Error;
use csv::Reader;
use std::fs;

pub fn process_csv_to_json(input: &str, output: &str) -> Result<(), Error> {
    let mut reader = Reader::from_path(input)?;
    let records: Vec<ParcelOrder> = reader
        .deserialize()
        .enumerate()
        .map(|(i, record)| {
            let mut r: ParcelOrder = record.expect("");
            r.index = i;
            r.site_code = r.dispatch_site.site_code();
            r
        })
        .inspect(|record| println!("{:?}, {:?}", record.index, record.order_number))
        .collect();

    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;
    Ok(())
}
