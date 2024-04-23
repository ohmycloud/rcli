use csv::{Reader, StringRecord};
use serde_json::Value;
use std::fs;

pub fn process_csv(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input_path)?;
    let mut ret = Vec::with_capacity(256);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record: StringRecord = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output_path, json)?;
    Ok(())
}
