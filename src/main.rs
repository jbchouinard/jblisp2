use std::path::PathBuf;

use rustyline::Editor;
use structopt::StructOpt;

use jbscheme::Interpreter;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HISTORY_FILE: &str = ".jbscheme_history";

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
    #[structopt(short, long)]
    interactive: bool,
}

fn main() {
    let opt = Opt::from_args();
    let mut interpreter = Interpreter::default();

    for file in &opt.files {
        if let Err(e) = interpreter.eval_file(&file) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    if opt.interactive || opt.files.is_empty() {
        repl(interpreter);
    }
}

fn repl(mut interpreter: Interpreter) {
    println!("jbscheme v{}", VERSION);
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(HISTORY_FILE);
    while let Ok(line) = rl.readline(">>> ") {
        rl.add_history_entry(line.as_str());
        match interpreter.eval_str("stdin", &line) {
            Ok(Some(val)) => println!("{}", val),
            Ok(None) => (),
            Err(err) => eprintln!("Unhandled {}", err),
        }
    }
    rl.save_history(HISTORY_FILE).unwrap();
}
