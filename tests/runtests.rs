use std::path::Path;

use jbscheme::Interpreter;

fn execfile<P: AsRef<Path>>(path: P) {
    let mut interpreter = Interpreter::default();
    interpreter.eval_file(path).unwrap();
}

#[test]
fn test_suite_lang() {
    execfile("tests/lang.jbscm")
}

#[test]
fn test_suite_string() {
    execfile("tests/string.jbscm")
}

#[test]
fn test_suite_math() {
    execfile("tests/math.jbscm")
}
