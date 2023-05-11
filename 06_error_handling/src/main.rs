use std::error;
use thiserror::Error;
use std::io::{self, BufRead, BufReader, stdin};
use colored::*;

// Ability to convert any error
type GenericError = Box<dyn error::Error>;
type GenericResult<T> = Result<T, GenericError>;

// Make it thread safe
type ThreadSafeGenericError = Box<dyn error::Error + Send + Sync + 'static>;
type ThreadSafeGenericResult<T> = Result<T, ThreadSafeGenericError>;


#[derive(Debug, Error)]
#[error("'{message}' on line {line_number}: '{line}'")]
pub struct ParseLineError{
    message: String,
    line: String,
    line_number: usize,
}


fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i32>> {
    let mut numbers = vec![];
    for (line_index, line_result) in file.lines().enumerate() {
        let line = line_result?;
        match line.parse::<i32>() {
            Ok(number) => numbers.push(number),
            Err(parse_error) => {
                return Err(Box::new(ParseLineError {
                    message: parse_error.to_string(),
                    line,
                    line_number: line_index + 1,
                }));
            }
        }
    }
    Ok(numbers)
}
fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);
    let numbers = read_numbers(&mut reader);
    match numbers {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(err) => {
            if let Some(io_error) = err.downcast_ref::<io::Error>() {
                eprintln!("{}: {}", "IO Error".yellow().underline(), io_error);
            } else if let Some(parse_error) = err.downcast_ref::<ParseLineError>() {
                eprintln!("{}: {}", "ParseLineError".yellow().bold(),  parse_error);
            } else {
                eprintln!("{}: {}", "Unknown Error".red().bold(), err);
            }
        }
    }
}
