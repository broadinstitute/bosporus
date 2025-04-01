use rustyline::{DefaultEditor, Editor};
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use crate::files;

pub(crate) struct Shell {
    editor: Editor<(), DefaultHistory>
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            editor: DefaultEditor::new().unwrap()
        }
    }

    pub fn run(&mut self) {
        if let Some(history_file) = files::history_file() {
            if let Err(error) = self.editor.load_history(&history_file) {
                println!("Error loading history: {}", error);
            }
        }
        loop {
            let readline = self.editor.readline("BOS> ");
            match readline {
                Ok(line) => {
                    if let Err(error) = self.editor.add_history_entry(line.as_str()) {
                        println!("Error adding to history: {}", error);
                    }
                    println!("You entered: {}", line);
                }
                Err(error) => {
                    match error {
                        ReadlineError::Io(_) => {}
                        ReadlineError::Eof => {}
                        ReadlineError::Interrupted => {}
                        ReadlineError::Errno(_) => {}
                        ReadlineError::WindowResized => {}
                        _ => {
                            println!("Error: {:?}", error);
                        }
                    }
                }
            }
        }
    }
}