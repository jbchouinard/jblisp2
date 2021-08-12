use std::path::Path;

use jibi::Interpreter;

fn run_jibi_test<P: AsRef<Path> + std::fmt::Debug>(path: P) {
    let mut interpreter = Interpreter::default();
    if let Err((_, e, _)) = interpreter.eval_file(path) {
        panic!("{}", e);
    }
}

macro_rules! jibitest {
    ( $name:ident ) => {
        #[test]
        fn $name() {
            run_jibi_test(format!("tests/test-{}.jibi", stringify!($name)));
        }
    };
}

jibitest!(base);
jibitest!(bindings);
jibitest!(eval);
jibitest!(functions);
jibitest!(integers);
jibitest!(lists);
jibitest!(strings);
jibitest!(stl_math);
jibitest!(stl_decimal);
