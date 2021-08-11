use std::path::Path;

use jibi::Interpreter;

fn execfile<P: AsRef<Path>>(path: P) {
    let mut interpreter = Interpreter::default();
    interpreter.eval_file(path).unwrap();
}

#[test]
fn tests_lang() {
    execfile("tests/lang.jibi")
}

#[test]
fn tests_string() {
    execfile("tests/string.jibi")
}

#[test]
fn tests_math() {
    execfile("tests/math.jibi")
}
