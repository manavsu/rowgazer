use csv::ReaderBuilder;
use std::boxed::Box;
use std::cmp::max;
use std::error::Error;
use std::result::Result;
use std::string::String;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path("customers-100.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

enum CSVEntry {
    String(String),
    Int(i32),
    Float(f32),
}

struct CSVData {
    data: Vec<Vec<CSVEntry>>,
    max_rows: usize,
    max_cols: usize,
}

fn read_csv(path: &String) -> Result<CSVData, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(path).unwrap();
    let mut csv_entries: Vec<Vec<CSVEntry>> = Vec::new();
    let mut max_rows = 0;
    let mut max_cols = 0;

    for record in rdr.records() {
        max_cols += 1;
        match record {
            Ok(rec) => {
                let mut row: Vec<CSVEntry> = Vec::new();
                max_rows = max_rows.max(rec.len());
                for field in rec.iter() {
                    row.push(parse_entry(field));
                }
                csv_entries.push(row);
            }
            Err(e) => eprintln!("Error reading record: {}", e),
        }
    }

    Ok(CSVData {
        data: csv_entries,
        max_rows,
        max_cols,
    })
}

fn parse_entry(entry: &str) -> CSVEntry {
    if let Ok(int_value) = entry.parse::<i32>() {
        CSVEntry::Int(int_value)
    } else if let Ok(float_value) = entry.parse::<f32>() {
        CSVEntry::Float(float_value)
    } else {
        CSVEntry::String(entry.to_string())
    }
}
