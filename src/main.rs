use std::path::{Path, PathBuf};
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use structopt::StructOpt;

use crate::builtin::add_builtins;
pub use crate::env::{JEnv, JEnvRef};
pub use crate::error::*;
pub use crate::eval::jeval;
pub use crate::primitives::*;
pub use crate::reader::parser::Parser;
use crate::reader::ReaderError;
pub use crate::repr::jrepr;
pub use crate::state::JState;

pub mod builtin;
pub mod env;
pub mod error;
pub mod eval;
pub mod primitives;
pub mod reader;
pub mod repr;
pub mod state;

const PRELUDE: &str = include_str!("../stl/prelude.jbscm");
const HISTORY_FILE: &str = ".jbscheme_history";
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
    let mut state = JState::default();
    let globals = make_globals(&mut state);

    for file in &opt.files {
        if let Err(je) = eval_file(&file, Rc::clone(&globals), &mut state) {
            eprintln!("{}: {}", file.to_str().unwrap(), je);
            std::process::exit(1);
        }
    }

    if opt.interactive || opt.files.is_empty() {
        repl(globals, &mut state);
    }
}

fn repl(globals: JEnvRef, state: &mut JState) {
    println!("jbscheme v{}", VERSION);
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(HISTORY_FILE);
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match eval_text(&line, Rc::clone(&globals), state) {
                    Ok(Some(val)) => println!("{}", jrepr(&val)),
                    Ok(None) => (),
                    Err(je) => eprintln!("Unhandled {}: {}", je.etype, je.emsg),
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

fn eval_text(text: &str, env: JEnvRef, state: &mut JState) -> Result<Option<JValRef>, JError> {
    let forms = Parser::new(text, state).parse_forms()?;
    let mut last_eval = None;
    for form in forms {
        last_eval = Some(jeval(form, Rc::clone(&env), state)?);
    }
    Ok(last_eval)
}

pub fn eval_file<P: AsRef<Path>>(
    path: P,
    env: JEnvRef,
    state: &mut JState,
) -> Result<Option<JValRef>, JError> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return Err(JError::new("FileError", &format!("{}", e))),
    };
    eval_text(&text, env, state)
}

fn make_globals(state: &mut JState) -> JEnvRef {
    let env = JEnv::default().into_ref();
    add_builtins(&env);
    if let Err(je) = eval_text(PRELUDE, Rc::clone(&env), state) {
        eprintln!("prelude: {}", je);
        std::process::exit(1);
    }
    env
}
