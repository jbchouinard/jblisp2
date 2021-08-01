use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::reader::tokenize::{Token, Tokenizer};
use crate::reader::ReaderError;

pub mod reader;

const HISTORY_FILE: &str = ".jblisp2_history";

fn jeval(s: String) -> Result<Vec<Token>, ReaderError> {
    Tokenizer::new(&s).tokenize()
}

fn jprint(t: &Result<Vec<Token>, ReaderError>) {
    println!("{:#?}", t);
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(HISTORY_FILE);
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                jprint(&jeval(line));
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_FILE).unwrap();
}
