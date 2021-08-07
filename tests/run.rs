use std::path::Path;

use jbscheme::Interpreter;

fn execfile<P: AsRef<Path>>(path: P) {
    let mut interpreter = Interpreter::default();
    interpreter.eval_file(path).unwrap();
}

#[test]
fn tests_lang() {
    execfile("tests/lang.jbscm")
}

#[test]
fn tests_string() {
    execfile("tests/string.jbscm")
}

#[test]
fn tests_math() {
    execfile("tests/math.jbscm")
}

fn hello() {
    // Create an interpreter pre-loaded with definitions for builtins, and constants,
    // lambdas and macros defined by the prelude.
    // (Interpreter::new() creates a bare interpreter, with empty globals.)
    let mut interpreter = Interpreter::default();
    match interpreter.eval_str("main.rs", r#"(print "Hello World!)"#) {
        Ok(Some(jval)) => println!("{}", jval),
        Ok(None) => (),
        Err(je) => eprintln!("{}", je),
    };
}
