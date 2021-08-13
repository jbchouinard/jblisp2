use std::path::Path;
use std::rc::Rc;

use crate::builtin::add_builtins;
use crate::builtin::add_reader_macros;
use crate::state::JState;
use crate::*;

pub const PRELUDE: &str = include_str!("prelude.jibi");

pub struct Interpreter {
    state: JState,
    globals: JEnvRef,
}

impl Default for interpreter::Interpreter {
    /// Create `jibi` interpreter pre-loaded with builtins and common definitions
    /// (the [prelude](PRELUDE)).
    fn default() -> Self {
        let mut interpreter = Self::new();
        add_reader_macros(&mut interpreter.state);
        interpreter.define_builtins();
        interpreter.exec_prelude();
        interpreter
    }
}

/// `jibi` Language Interpreter
///
/// Each Interpreter has its own state and global environment, and has methods
/// to construct (interned) `jibi` values and procedures, define global variables,
/// call procedures, evaluate expressions, and execute code.
impl Interpreter {
    /// Create a bare `jibi` [`Interpreter`], with no builtins or [prelude](PRELUDE);
    /// call [`Interpreter::default`] instead if you want those.
    pub fn new() -> Self {
        Self {
            state: JState::new(),
            globals: Rc::new(JEnv::default()),
        }
    }
    /// Create global bindings for builtin functions and macros.
    pub(crate) fn define_builtins(&mut self) {
        add_builtins(&self.globals, &mut self.state);
    }
    /// Execute the `jibi` [prelude](PRELUDE), which defines common constants, procedures
    /// and macros.
    pub(crate) fn exec_prelude(&mut self) {
        if let Err(exc) = self.eval_str("#PRELUDE", PRELUDE) {
            Interpreter::print_exc(exc);
            std::process::exit(1);
        }
    }

    /// Evaluate a stream of tokens and return the value of the last expression,
    /// (or None if the program contains no expressions).
    pub fn eval_tokens(
        &mut self,
        tokens: Box<dyn TokenIter>,
    ) -> Result<Option<JValRef>, JException> {
        self.state.eval_tokens(tokens, Rc::clone(&self.globals))
    }

    /// Evaluate a `jibi` script, and return the value of its last expression
    /// (or None if the program contains no expressions).
    ///
    /// * `name`: Name used to report errors in the program (e.g. filename, "stdin").
    /// * `program`: One or more `jibi` expressions.
    pub fn eval_str(&mut self, name: &str, program: &str) -> Result<Option<JValRef>, JException> {
        self.state.eval_str(name, program, Rc::clone(&self.globals))
    }
    /// Evaluate a `jibi` script file, and return the value of the last expression
    /// or None if the file contains no expressions).
    //
    /// * `path`: Path to script file.
    pub fn eval_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<JValRef>, JException> {
        self.state.eval_file(path, Rc::clone(&self.globals))
    }
    /// Evaluate a `jibi` expression.
    /// (All values are expressions, primitive types evaluates to themselves.)
    pub fn eval(&mut self, expr: JValRef) -> JResult {
        eval(expr, Rc::clone(&self.globals), &mut self.state)
    }
    /// Call a named `jibi` procedure.
    ///
    /// * `name`: Name of procedure (looked up in globals).
    /// * `args`: Vector of argument values.
    pub fn call(&mut self, name: &str, args: Vec<JValRef>) -> JResult {
        let proc = self.globals.try_lookup(name)?;
        let mut sexpr = vec![proc];
        for arg in args {
            sexpr.push(self.state.quote(arg));
        }
        let sexpr = self.state.list(sexpr);
        eval(sexpr, Rc::clone(&self.globals), &mut self.state)
    }

    /// Print exception and traceback.
    pub fn print_exc((pos, err, mut tb): JException) {
        eprintln!("Traceback:");
        while !tb.is_empty() {
            eprintln!("  {}", tb.pop().unwrap());
        }
        eprintln!("  File \"{}\", line {}", pos.filename, pos.lineno);
        eprintln!("{}", err);
    }

    /// Create a global binding (variable definition).
    ///
    /// * `name`: Name to bind. It is possible to bind names which are not valid
    ///           `jibi` symbols, they will not be accessible in `jibi` code.
    /// * `val`: A `jibi` value.
    pub fn def(&mut self, name: &str, val: JValRef) {
        self.globals.define(name, val)
    }
    /// Get value of binding in global environment.
    ///
    /// * `name`: Name to lookup. It is possible to bind names which are not valid
    ///           `jibi` symbols, they will not be accessible in `jibi` code.
    pub fn lookup(&self, name: &str) -> Option<JValRef> {
        self.globals.lookup(name)
    }
    /// Construct a `jibi` `nil` (always interned).
    pub fn nil(&mut self) -> JValRef {
        self.state.nil()
    }
    /// Construct a `jibi` `bool` (always interned).
    pub fn bool(&mut self, b: bool) -> JValRef {
        self.state.bool(b)
    }
    pub fn int(&mut self, n: JTInt) -> JValRef {
        self.state.int(n)
    }
    /// Construct a `jibi` `symbol` (always interned).
    pub fn symbol(&mut self, s: String) -> JValRef {
        self.state.symbol(s)
    }
    /// Construct a `jibi` `string` (may be interned).
    pub fn string(&mut self, s: String) -> JValRef {
        self.state.string(s)
    }
    /// Construct a `jibi` `quote`.
    pub fn quote(&mut self, v: JValRef) -> JValRef {
        self.state.quote(v)
    }
    /// Construct a `jibi` list (linked list made from `pair` and terminated
    /// by `nil`).
    pub fn list(&mut self, v: Vec<JValRef>) -> JValRef {
        self.state.list(v)
    }
    /// Construct a `jibi` `pair` (cons cell).
    pub fn pair(&mut self, left: JValRef, right: JValRef) -> JValRef {
        self.state.pair(left, right)
    }
    /// Construct a `jibi` `error` value.
    ///
    /// Note that [`JError`] can be found both in [`Ok`]`(`[`JValRef`]`)`, as first class values
    /// that can be passed around in `jibi`, and in [`Err`]`(`[`JError`]`)` when it is
    /// `raise`'d by `jibi` code, or due to parsing or evaluation errors.
    pub fn error(&mut self, kind: JErrorKind, reason: &str) -> JValRef {
        self.state.error(kind, reason)
    }
    /// Construct a `jibi` `lambda`.
    pub fn lambda(&mut self, params: Vec<String>, body: Vec<JValRef>) -> JResult {
        self.state
            .lambda(Rc::clone(&self.globals), params, body, None)
    }
    /// Construct a `jibi` `procmacro`.
    pub fn procmacro(&mut self, params: Vec<String>, body: Vec<JValRef>) -> JResult {
        self.state
            .procmacro(Rc::clone(&self.globals), params, body, None)
    }
    /// Define a `jibi` builtin procedure.
    pub fn builtin<F>(&mut self, name: String, f: F) -> JValRef
    where
        F: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
    {
        let v = self.state.builtin(name.clone(), Rc::new(f));
        self.def(&name, Rc::clone(&v));
        v
    }
    /// Define a `jibi` builtin special form.
    pub fn specialform<F>(&mut self, name: String, f: F) -> JValRef
    where
        F: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
    {
        let v = self.state.builtin(name.clone(), Rc::new(f));
        self.def(&name, Rc::clone(&v));
        v
    }
}
