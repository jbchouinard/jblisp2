use std::path::Path;

use jibi::Interpreter;

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
