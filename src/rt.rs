use std::io::Write;
use rustyline::{DefaultEditor, Editor};
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use crate::files;

pub(crate) struct Runtime {
    active: bool,
    editor: Editor<(), DefaultHistory>
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            active: true,
            editor: DefaultEditor::new().unwrap()
        }
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn offer_stop(&mut self) {
        let mut said_yes = false;
        let mut said_no = false;
        let mut attempts = 0;
        while !said_yes && !said_no && attempts < 3 {
            let mut buffer = String::new();
            print!("Quit? (y/n): ");
            std::io::stdout().flush().unwrap();
            match std::io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    if buffer.trim().to_lowercase() == "y" {
                        said_yes = true;
                    } else {
                        said_no = true;
                    }
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                }
            }
            attempts += 1;
        }
        if said_no {
            println!("Continuing...");
        } else {
            println!("Exiting...");
            self.stop();
        }
    }

    pub fn interactive(&mut self) {
        if let Some(history_file) = files::history_file() {
            if let Err(error) = self.editor.load_history(&history_file) {
                println!("Error loading history: {}", error);
            }
        }
        while self.active {
            let readline = self.editor.readline("BOS> ");
            match readline {
                Ok(line) => {
                    if let Err(error) = self.editor.add_history_entry(line.as_str()) {
                        println!("Error adding to history: {}", error);
                    }
                    if let Err(error) = self.interpret(line.as_str()) {
                        println!("Error interpreting line: {}", error);
                    }
                }
                Err(error) => {
                    match error {
                        ReadlineError::Io(error) => { println!("IO Error: {}", error); }
                        ReadlineError::Eof => { self.offer_stop()}
                        ReadlineError::Interrupted => { self.offer_stop() }
                        ReadlineError::Errno(error) => { println!("Errno: {}", error); }
                        ReadlineError::WindowResized => { println!("Window resized"); }
                        _ => { println!("Error: {}", error); }
                    }
                }
            }
        }
    }
    pub fn interpret(&mut self, line: &str) -> Result<(), String> {
        let line = line.trim();
        match line {
            "exit" | "quit" => {
                self.offer_stop();
                Ok(())
            }
            _ => {
                match line.split_once("=") {
                    Some((var, value)) => {
                        let var = var.trim();
                        let value = value.trim();
                        println!("Setting variable {} to {}", var.trim(), value.trim());
                        Ok(())
                    }
                    None => {
                        match line.split_once(|c: char| c.is_whitespace()) {
                            Some((command, args)) => {
                                let command = command.trim();
                                let args = args.trim();
                                println!("Command: {}, Args: {}", command, args);
                                Ok(())
                            }
                            None => {
                                Err(format!("Cannot parse: {}", line))
                            }
                        }
                    }
                }
            }
        }
    }
}

