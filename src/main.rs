use std::path::{Path, PathBuf};
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use structopt::StructOpt;

use crate::builtin::add_builtins;
pub use crate::env::{JEnv, JEnvRef};
pub use crate::error::*;
pub use crate::eval::jeval;
pub use crate::reader::parser::Parser;
use crate::reader::ReaderError;
pub use crate::repr::jrepr;
pub use crate::types::*;

pub mod builtin;
pub mod env;
pub mod error;
pub mod eval;
pub mod reader;
pub mod repr;
pub mod types;

const PRELUDE: &str = include_str!("prelude.jblisp");
const HISTORY_FILE: &str = ".jblisp2_history";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
    #[structopt(short, long)]
    interactive: bool,
}

fn main() {
    let opt = Opt::from_args();
    let globals = make_globals();

    for file in &opt.files {
        if let Some(JValue::Error(je)) = eval_file(&file, Rc::clone(&globals)) {
            eprintln!("{}: {}", file.to_str().unwrap(), je);
            std::process::exit(1);
        }
    }

    if opt.interactive || opt.files.is_empty() {
        repl(globals);
    }
}

fn eval_text(text: &str, env: JEnvRef) -> Option<JValue> {
    let forms = match Parser::new(text).parse_forms() {
        Ok(forms) => forms,
        Err(re) => return Some(JValue::Error(re.into())),
    };
    let mut last_eval = None;
    for form in forms {
        match jeval(form, Rc::clone(&env)) {
            Ok(val) => last_eval = Some(val),
            Err(je) => return Some(JValue::Error(je)),
        }
    }
    last_eval
}

fn eval_file<P: AsRef<Path>>(path: P, env: JEnvRef) -> Option<JValue> {
    eval_text(&std::fs::read_to_string(path).unwrap(), env)
}

fn make_globals() -> JEnvRef {
    let env = JEnv::default().into_ref();
    add_builtins(&env);
    if let Some(JValue::Error(je)) = eval_text(PRELUDE, Rc::clone(&env)) {
        eprintln!("prelude: {}", je);
        std::process::exit(1);
    }
    env
}

fn repl(globals: JEnvRef) {
    println!("jblisp2 v{}", VERSION);
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(HISTORY_FILE);
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if let Some(val) = eval_text(&line, Rc::clone(&globals)) {
                    println!("{}", jrepr(&val));
                }
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
