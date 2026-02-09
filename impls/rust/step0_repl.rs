use rustyline::DefaultEditor;
use rustyline::config::{BellStyle, Builder};
use rustyline::error::ReadlineError;

static MAX_HIST_SIZE: usize = 128;

// Read and return user input using rustyline, exits on read failure
fn read_line(reader: &mut DefaultEditor) -> String {
    match reader.readline("user> ") {
        Ok(input) => input,

        Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
            // Ctrl+D or Ctrl+C received, exit cleanly
            std::process::exit(0);
        },

        Err(error) => {
            println!("Read error: {}", error.to_string());
            std::process::exit(1);
        }
    }
}

fn eval(input: String) -> String {
    return input;
}

fn rep(input: String) {
    let output = eval(input);
    println!("{}", output);
}

fn main() {
    let mut reader = DefaultEditor::with_config(
        Builder::new()
            .max_history_size(MAX_HIST_SIZE).unwrap()
            .auto_add_history(true)
            .bell_style(BellStyle::None)
            .build()
    ).unwrap();
    
    loop {
        let input = read_line(&mut reader);

        rep(input);
    }
}
