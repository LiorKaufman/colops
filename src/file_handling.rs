// src/file_handling.rs
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FileType {
    CSV,
    JSON,
}

pub fn read_input_file(
    file_path: &PathBuf,
) -> Result<(FileType, Vec<Vec<String>>), Box<dyn Error>> {
    let file_extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| format!("Invalid file extension: {}", file_path.display()))?;

    let file_type = match file_extension.to_lowercase().as_str() {
        "csv" => FileType::CSV,
        "json" => FileType::JSON,
        _ => {
            return Err(format!(
                "Unsupported file type: {}. Only CSV and JSON files are supported.",
                file_path.display()
            )
            .into());
        }
    };

    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);
    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut row = Vec::new();

        for value in record.iter() {
            row.push(value.to_string());
        }

        records.push(row);
    }

    if records.is_empty() {
        return Err("File has no records".into());
    }

    let num_cols = records[0].len();

    if headers.len() != num_cols {
        return Err("Header count does not match column count".into());
    }

    Ok((file_type, records))
}
