use rustyline::error::ReadlineError;
use rustyline::Editor;

pub use crate::env::JEnv;
pub use crate::error::*;
pub use crate::eval::jeval;
pub use crate::reader::parser::Parser;
pub use crate::repr::jrepr;

use crate::reader::ReaderError;
pub use crate::types::*;

pub mod builtin;
pub mod env;
pub mod error;
pub mod eval;
pub mod reader;
pub mod repr;
pub mod types;

const HISTORY_FILE: &str = ".jblisp2_history";

fn repl() {
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(HISTORY_FILE);
    let mut env = JEnv::default();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let expr = Parser::new(&line).parse();
                println!("{}", jrepr(&jeval(expr, &mut env).into()));
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

fn main() {
    repl();
}
