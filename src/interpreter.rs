use std::path::Path;
use std::rc::Rc;

use crate::builtin::add_builtins;
use crate::error::JError;
use crate::state::JState;
use crate::*;

pub const PRELUDE: &str = include_str!("prelude.jbscm");

pub struct Interpreter {
    state: JState,
    globals: JEnvRef,
}

impl Default for interpreter::Interpreter {
    /// Create `jbscheme` interpreter pre-loaded with builtins and common definitions
    /// (the [prelude](PRELUDE)).
    fn default() -> Self {
        let mut interpreter = Self::new();
        interpreter.define_builtins();
        interpreter.exec_prelude();
        interpreter
    }
}

/// `jbscheme` Language Interpreter
///
/// Each Interpreter has its own state and global environment, and has methods
/// to construct (interned) `jbscheme` values and procedures, define global variables,
/// call procedures, evaluate expressions, and execute code.
impl Interpreter {
    /// Create a bare `jbscheme` [`Interpreter`], with no builtins or [prelude](PRELUDE);
    /// call [`Interpreter::default`] instead if you want those.
    pub fn new() -> Self {
        Self {
            state: JState::new(),
            globals: Rc::new(JEnv::default()),
        }
    }
    /// Create global bindings for builtin functions and macros.
    pub fn define_builtins(&mut self) {
        add_builtins(&self.globals, &mut self.state);
    }
    /// Execute the `jbscheme` [prelude](PRELUDE), which defines common constants, procedures
    /// and macros.
    pub fn exec_prelude(&mut self) {
        if let Err(je) = self.eval_str("#PRELUDE", PRELUDE) {
            eprintln!("{}", je);
            std::process::exit(1);
        }
    }
    /// Evaluate a `jbscheme` script, and return the value of its last expression
    /// (or None if the program contains no expressions).
    ///
    /// * `name`: Name used to report errors in the program (e.g. filename, "stdin").
    /// * `program`: One or more `jbscheme` expressions.
    pub fn eval_str(&mut self, name: &str, program: &str) -> Result<Option<JValRef>, JError> {
        self.state.eval_str(name, program, Rc::clone(&self.globals))
    }
    /// Evaluate a `jbscheme` script file, and return the value of the last expression
    /// or None if the file contains no expressions).
    //
    /// * `path`: Path to script file.
    pub fn eval_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<JValRef>, JError> {
        self.state.eval_file(path, Rc::clone(&self.globals))
    }

    /// Evaluate a `jbscheme` expression.
    /// (All values are expressions, primitive types evaluates to themselves.)
    pub fn eval(&mut self, expr: JValRef) -> JResult {
        eval(expr, Rc::clone(&self.globals), &mut self.state)
    }
    /// Call a named `jbscheme` procedure.
    ///
    /// * `name`: Name of procedure (looked up in globals).
    /// * `args`: Vector of argument values.
    pub fn call(&mut self, name: &str, args: Vec<JValRef>) -> JResult {
        let proc = self.globals.try_lookup(name)?;
        let mut sexpr = vec![proc];
        sexpr.extend(args);
        let sexpr = self.state.list(sexpr);
        eval(sexpr, Rc::clone(&self.globals), &mut self.state)
    }
    /// Create a global binding (variable definition).
    ///
    /// * `name`: Name to bind. It is possible to bind names which are not valid
    ///           `jbscheme` symbols, they will not be accessible in `jbscheme` code.
    /// * `val`: A `jbscheme` value.
    pub fn def(&mut self, name: &str, val: JValRef) {
        self.globals.define(name, val)
    }
    /// Get value of binding in global environment.
    ///
    /// * `name`: Name to lookup. It is possible to bind names which are not valid
    ///           `jbscheme` symbols, they will not be accessible in `jbscheme` code.
    pub fn lookup(&self, name: &str) -> Option<JValRef> {
        self.globals.lookup(name)
    }
    /// Construct a `jbscheme` `nil` (always interned).
    pub fn jnil(&mut self) -> JValRef {
        self.state.nil()
    }
    /// Construct a `jbscheme` `bool` (always interned).
    pub fn jbool(&mut self, b: bool) -> JValRef {
        self.state.bool(b)
    }
    /// Construct a `jbscheme` `int` (may be interned).
    pub fn jint(&mut self, n: JTInt) -> JValRef {
        self.state.int(n)
    }
    /// Construct a `jbscheme` `symbol` (always interned).
    pub fn jsymbol(&mut self, s: String) -> JValRef {
        self.state.symbol(s)
    }
    /// Construct a `jbscheme` `string` (may be interned).
    pub fn jstring(&mut self, s: String) -> JValRef {
        self.state.string(s)
    }
    /// Construct a `jbscheme` `quote`.
    pub fn jquote(&mut self, v: JValRef) -> JValRef {
        self.state.quote(v)
    }
    /// Construct a `jbscheme` list (linked list made from `pair` and terminated
    /// by `nil`).
    pub fn jlist(&mut self, v: Vec<JValRef>) -> JValRef {
        self.state.list(v)
    }
    /// Construct a `jbscheme` `pair` (cons cell).
    pub fn jpair(&mut self, left: JValRef, right: JValRef) -> JValRef {
        self.state.pair(left, right)
    }
    /// Construct a `jbscheme` `error` value.
    ///
    /// Note that [`JError`] can be found both in [`Ok`]`(`[`JValRef`]`)`, as first class values
    /// that can be passed around in `jbscheme`, and in [`Err`]`(`[`JError`]`)` when it is
    /// `raise`'d by `jbscheme` code, or due to parsing or evaluation errors.
    pub fn jerrorval(&mut self, err: JError) -> JValRef {
        self.state.error(err)
    }
    /// Construct a `jbscheme` `lambda`.
    pub fn jlambda(&mut self, params: Vec<String>, body: JValRef) -> JResult {
        self.state.lambda(Rc::clone(&self.globals), params, body)
    }
    /// Construct a `jbscheme` `macro`.
    pub fn jmacro(&mut self, params: Vec<String>, body: JValRef) -> JResult {
        self.state.lmacro(Rc::clone(&self.globals), params, body)
    }
    /// Define a `jbscheme` builtin procedure.
    pub fn jbuiltin<F>(&mut self, name: String, f: F) -> JValRef
    where
        F: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
    {
        let v = self.state.builtin(name.clone(), Rc::new(f));
        self.def(&name, Rc::clone(&v));
        v
    }
    /// Define a `jbscheme` builtin special form.
    pub fn jspecialform<F>(&mut self, name: String, f: F) -> JValRef
    where
        F: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
    {
        let v = self.state.builtin(name.clone(), Rc::new(f));
        self.def(&name, Rc::clone(&v));
        v
    }
}
