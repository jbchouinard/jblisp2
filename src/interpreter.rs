use std::path::Path;
use std::rc::Rc;

use crate::builtin::add_builtins;
use crate::error::JError;
use crate::state::JState;
use crate::*;

const PRELUDE: &str = include_str!("prelude.jbscm");

pub struct Interpreter {
    state: JState,
    globals: JEnvRef,
}

impl Default for interpreter::Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// JB Scheme Interpreter.
impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            state: JState::new(),
            globals: Rc::new(JEnv::default()),
        };
        interpreter.setup_globals();
        interpreter
    }
    fn setup_globals(&mut self) {
        add_builtins(&self.globals, &mut self.state);
        if let Err(je) = self.eval_str("prelude", PRELUDE) {
            eprintln!("{}", je);
            std::process::exit(1);
        }
    }
    pub fn eval_str(&mut self, filename: &str, program: &str) -> Result<Option<JValRef>, JError> {
        self.state
            .eval_str(filename, program, Rc::clone(&self.globals))
    }
    pub fn eval_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<JValRef>, JError> {
        self.state.eval_file(path, Rc::clone(&self.globals))
    }
    pub fn call(_name: &str, _args: Vec<JValRef>) -> JResult {
        todo!()
    }
}
