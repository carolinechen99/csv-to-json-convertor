use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::error::Error;

use csv::ReaderBuilder;
use serde_json::to_writer_pretty;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} <input_csv_file> <output_json_file>", args[0]);
    }

    let input_csv_file = &args[1];
    let output_json_file = &args[2];

    let csv_reader = BufReader::new(File::open(input_csv_file)?);
    let mut reader = ReaderBuilder::new().from_reader(csv_reader);

    let headers = reader.headers()?.clone();

    let json_writer = BufWriter::new(File::create(output_json_file)?);

    let mut records: Vec<HashMap<String, String>> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut record_map = HashMap::new();
        for (header, field) in headers.iter().zip(record.iter()) {
            record_map.insert(header.to_string(), field.to_string());
        }
        records.push(record_map);
    }

    to_writer_pretty(json_writer, &records)?;

    println!("CSV file '{input_csv_file}' has been converted to JSON file '{output_json_file}'");

    Ok(())
}

