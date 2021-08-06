use std::path::PathBuf;
use std::thread;

use rustyline::Editor;
use structopt::StructOpt;

use jbscheme::Interpreter;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

const HISTORY_FILE: &str = ".jbscheme_history";

fn repl(mut interpreter: Interpreter) {
    println!("jbscheme v{}", VERSION);
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(HISTORY_FILE);
    while let Ok(line) = rl.readline(">>> ") {
        rl.add_history_entry(line.as_str());
        match interpreter.eval_str("#STDIN", &line) {
            Ok(Some(val)) => println!("{}", val),
            Ok(None) => (),
            Err(err) => eprintln!("Unhandled {}", err),
        }
    }
    rl.save_history(HISTORY_FILE).unwrap();
}

// HACK: No tail call optimization yet, so just using a big stack size for now to make
// recursive functions a bit less bad
const STACK_SIZE: usize = 64 * 1024 * 1024;

fn main() {
    thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
