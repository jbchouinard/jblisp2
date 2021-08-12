use jibi::Interpreter;

macro_rules! jibitest {
    ( $name:ident ) => {
        #[test]
        fn $name() {
            if let Err((_, e, _)) = Interpreter::default().eval_file(format!(
                "tests/{}.jibi",
                stringify!($name).replace("_", "-")
            )) {
                panic!("{}", e);
            }
        }
    };
}

jibitest!(test_lang_base);
jibitest!(test_lang_bindings);
jibitest!(test_lang_eval);
jibitest!(test_lang_functions);
jibitest!(test_lang_integers);
jibitest!(test_lang_lists);
jibitest!(test_lang_strings);
jibitest!(test_stl_decimal);
jibitest!(test_stl_math);
