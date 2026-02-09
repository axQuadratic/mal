use rustyline::DefaultEditor;
use rustyline::config::{BellStyle, Builder};
use rustyline::error::ReadlineError;

use crate::error::{ReadError, MalError};

// Handles capturing input from stdin
pub struct Reader {
    prompt: String,
    
    editor: DefaultEditor
}
    
impl Reader {
    pub fn init(prompt: &str) -> Result<Self, MalError> {
        let Ok(rl) = DefaultEditor::with_config(
            Builder::new()
                .auto_add_history(true)
                .bell_style(BellStyle::None)
                .build()
        ) else {
            return Err("Failed to aquire stdin".into());
        };

        let reader = Reader {
            prompt: String::from(prompt),
            editor: rl
        };

        Ok(reader)
    }
    
    // Read and return input using rustyline
    pub fn read_line(&mut self) -> Result<String, ReadError> {
        match self.editor.readline(&self.prompt) {
            Ok(input) => Ok(input),

            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                // Ctrl+D or Ctrl+C received
                return Err(ReadError::Interrupt);
            },

            Err(error) => {
                return Err(ReadError::Failure(error.to_string()));
            }
        }
    }
}
