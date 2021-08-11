use jibi::Interpreter;

fn main() {
    // Create an interpreter pre-loaded with definitions for builtins; and constants,
    // lambdas and macros defined by the prelude.
    // (Interpreter::new() instead creates a bare interpreter, with empty globals.)
    let mut interpreter = Interpreter::default();
    match interpreter.eval_str("hello.rs", r#"(print "Hello World!")"#) {
        Ok(Some(jval)) => println!("{}", jval),
        Ok(None) => (),
        Err(exc) => Interpreter::print_exc(exc),
    };
}
