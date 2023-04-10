use clap::{builder::PossibleValue, ValueEnum};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
pub mod file_handling;
use crate::file_handling::{read_input_file, FileType};

use clap::Parser;

pub type MyResult<T> = Result<T, Box<dyn Error>>;
/// A custom type that can be either a string or a number
#[derive(Debug, PartialEq, Clone)]
pub enum StringOrNumber {
    String(String),
    Number(i32),
}
impl StringOrNumber {
    pub fn as_str(&self) -> &str {
        match self {
            StringOrNumber::String(s) => s.as_str(),
            StringOrNumber::Number(n) => n.to_string().as_str(),
        }
    }

    pub fn as_number(&self) -> Option<u32> {
        match self {
            StringOrNumber::String(_) => None,
            StringOrNumber::Number(n) => Some(*n),
        }
    }
}
impl FromStr for StringOrNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(num) => Ok(StringOrNumber::Number(num)),
            Err(_) => Ok(StringOrNumber::String(s.to_owned())),
        }
    }
}

impl ValueEnum for StringOrNumber {
    fn value_variants() -> &'static [Self] {
        static VARIANTS: [StringOrNumber; 2] = [
            StringOrNumber::String(String::new()),
            StringOrNumber::Number(0),
        ];
        &VARIANTS
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            StringOrNumber::String(value) => Some(PossibleValue::Single(value.clone())),
            StringOrNumber::Number(value) => Some(PossibleValue::Single(value.to_string())),
        }
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        Self::value_variants()
            .iter()
            .find(|v| {
                v.to_possible_value()
                    .expect("ValueEnum::value_variants contains only values with a corresponding ValueEnum::to_possible_value")
                    .matches(input, ignore_case)
            })
            .cloned()
            .ok_or_else(|| format!("invalid variant: {input}"))
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Colops {
    /// Sets the input file (csv or json)
    #[arg(short = 'f', long, value_name = "FILE")]
    pub input_file: PathBuf,

    /// Specifies the column name or index
    #[arg(short = 'c', long, value_name = "COLUMN")]
    pub column: StringOrNumber,

    /// Specifies the operation to perform
    #[arg(short = 'o', long)]
    pub operation: String,
}

pub fn get_args() -> MyResult<Colops> {
    Ok(Colops::parse())
}

pub fn process_file(file_path: &PathBuf, column_name: &str) -> Result<(), Box<dyn Error>> {
    match read_input_file(file_path) {
        Ok((file_type, records)) => {
            println!("File type: {:?}", file_type);
            println!("Number of records: {}", records.len());

            // Find the index of the chosen column
            let headers = &records[0];
            let column_index = headers.iter().position(|h| h == column_name);

            match column_index {
                Some(col_idx) => {
                    // Print the values in the chosen column
                    println!("Values in column \"{}\":", column_name);
                    for row in &records[1..] {
                        println!("{}", row[col_idx]);
                    }
                }
                None => {
                    eprintln!("Column \"{}\" not found in file", column_name);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }

    Ok(())
}

pub fn run(colops: Colops) -> MyResult<()> {
    // Access the input file
    let input_file = colops.input_file;
    println!("Input file: {}", input_file.display());

    // Access the column
    let column = colops.column;
    println!("Column: {}", column);

    // Access the operation
    let operation = colops.operation;
    println!("Operation: {}", operation);

    // Continued program logic goes here...
    process_file(&input_file)?;

    Ok(())
}

// Add this at the bottom of the lib.rs file

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn empty_command() {
        let empty_args: Vec<String> = vec!["colops".to_string()];
        let result = Colops::try_parse_from(empty_args.iter());
        assert!(
            result.is_err(),
            "An error should be returned for empty command"
        );
    }

    #[test]
    fn valid_arguments() {
        let args: Vec<String> = vec![
            "colops".to_string(),
            "-f".to_string(),
            "input.csv".to_string(),
            "-c".to_string(),
            "name".to_string(),
            "-o".to_string(),
            "distinct".to_string(),
        ];
        let colops =
            Colops::try_parse_from(args.iter()).expect("Valid arguments should parse successfully");
        assert_eq!(colops.input_file, PathBuf::from("input.csv"));
        assert_eq!(colops.column, "name");
        assert_eq!(colops.operation, "distinct");
    }
}
