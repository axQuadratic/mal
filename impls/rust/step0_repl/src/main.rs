use std::process::ExitCode;

mod error;
mod repl;

use error::ReadError;
use repl::Reader;

fn eval(input: String) -> String {
    return input;
}

fn rep(input: String) {
    let output = eval(input);
    println!("{}", output);
}

fn main() -> ExitCode {
    let mut reader = Reader::init("user> ").unwrap();
    
    loop {
        let input = match reader.read_line() {
            Ok(input) => input,

            Err(ReadError::Interrupt) => {
                // Ctrl+C or Ctrl+D received, exit gracefully
                return ExitCode::SUCCESS;
            },

            Err(ReadError::Failure(error)) => {
                // Other failure, print it and exit non-gracefully
                println!("Error reading stdin: {}", error);
                return ExitCode::FAILURE;
            }
        };

        rep(input);
    }
}
