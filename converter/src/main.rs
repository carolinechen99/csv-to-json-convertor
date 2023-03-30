use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    age: u32,
    email: String,
}

fn csv_to_json(file_path: &str) -> Result<Value> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut records = csv::Reader::from_reader(reader).deserialize::<Data>();

    let mut json_data: Vec<Value> = vec![];
    for record in records {
        let data = record?;
        json_data.push(serde_json::to_value(data)?);
    }

    Ok(serde_json::to_value(json_data)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file_path = Path::new("data.csv");
    let output_file_path = Path::new("data.json");
    let json_data = csv_to_json(input_file_path.to_str().unwrap())?;

    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(json_data.to_string().as_bytes())?;

    Ok(())
}

