use std::path::PathBuf;
use std::str::FromStr;
use std::thread;

use home::home_dir;
use lazy_static::lazy_static;
use rustyline::Editor;
use structopt::StructOpt;

use jbscheme::Interpreter;
use jbscheme::{Token, TokenError, TokenValidator};

const VERSION: &str = env!("CARGO_PKG_VERSION");

// HACK: No tail call optimization yet, so just using a big stack size for now to make
// recursive functions a bit less bad
const STACK_SIZE: usize = 64 * 1024 * 1024;

lazy_static! {
    static ref HISTORY_FILE: PathBuf = match home_dir() {
        Some(mut p) => {
            p.push(".jbscheme_history");
            p
        }
        None => {
            eprintln!("could not locate home dir, saving history to current dir");
            PathBuf::from_str(".jbscheme_history").unwrap()
        }
    };
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
    #[structopt(short, long)]
    interactive: bool,
}

fn _main() {
    let Opt { files, interactive } = Opt::from_args();

    let mut interpreter = Interpreter::default();

    for file in &files {
        if let Err(e) = interpreter.eval_file(&file) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    if interactive || files.is_empty() {
        repl(interpreter);
    }
}

fn repl(mut interpreter: Interpreter) {
    println!("jbscheme v{}", VERSION);
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(&*HISTORY_FILE);

    loop {
        match get_tokens(&mut rl) {
            Ok(tokens) => match interpreter.eval_tokens("#STDIN", Box::new(tokens.into_iter())) {
                Ok(Some(val)) => println!("{}", val),
                Ok(None) => (),
                Err(err) => eprintln!("Unhandled {}", err),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn readline(rl: &mut Editor<()>, prompt: &str) -> String {
    let input = match rl.readline(prompt) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    rl.add_history_entry(&input);
    let _ = rl.save_history(&*HISTORY_FILE);
    input
}

/// Get tokens that looks like they form a complete expression (balanced parens)
/// in multiple lines of input if necessary.
fn get_tokens(rl: &mut Editor<()>) -> Result<Vec<Token>, TokenError> {
    let mut validator = TokenValidator::new();
    let mut input = readline(rl, ">>> ");
    loop {
        match validator.input(input.to_string()) {
            Ok(Some(v)) => return Ok(v),
            Ok(None) => (),
            Err(e) => return Err(e),
        }
        input = readline(rl, "... ");
    }
}

fn main() {
    thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
