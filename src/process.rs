use crate::Player;
use csv::Reader;
use std::fs;

pub fn process_csv(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input_path)?;
    let mut ret = Vec::with_capacity(256);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output_path, json)?;
    Ok(())
}
