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
        sexpr.extend(args);
        let sexpr = self.state.jlist(sexpr);
        eval(sexpr, Rc::clone(&self.globals), &mut self.state)
    }
    pub fn print_exc((pos, err, tb): JException) {
        eprintln!("Traceback:");
        for tbf in tb {
            eprintln!("  {}", tbf);
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
    pub fn jnil(&mut self) -> JValRef {
        self.state.jnil()
    }
    /// Construct a `jibi` `bool` (always interned).
    pub fn jbool(&mut self, b: bool) -> JValRef {
        self.state.jbool(b)
    }
    pub fn jint(&mut self, n: JTInt) -> JValRef {
        self.state.jint(n)
    }
    /// Construct a `jibi` `symbol` (always interned).
    pub fn jsymbol(&mut self, s: String) -> JValRef {
        self.state.jsymbol(s)
    }
    /// Construct a `jibi` `string` (may be interned).
    pub fn jstring(&mut self, s: String) -> JValRef {
        self.state.jstring(s)
    }
    /// Construct a `jibi` `quote`.
    pub fn jquote(&mut self, v: JValRef) -> JValRef {
        self.state.jquote(v)
    }
    /// Construct a `jibi` list (linked list made from `pair` and terminated
    /// by `nil`).
    pub fn jlist(&mut self, v: Vec<JValRef>) -> JValRef {
        self.state.jlist(v)
    }
    /// Construct a `jibi` `pair` (cons cell).
    pub fn jpair(&mut self, left: JValRef, right: JValRef) -> JValRef {
        self.state.jpair(left, right)
    }
    /// Construct a `jibi` `error` value.
    ///
    /// Note that [`JError`] can be found both in [`Ok`]`(`[`JValRef`]`)`, as first class values
    /// that can be passed around in `jibi`, and in [`Err`]`(`[`JError`]`)` when it is
    /// `raise`'d by `jibi` code, or due to parsing or evaluation errors.
    pub fn jerrorval(&mut self, kind: JErrorKind, reason: &str) -> JValRef {
        self.state.jerrorval(kind, reason)
    }
    /// Construct a `jibi` `lambda`.
    pub fn jlambda(&mut self, params: Vec<String>, body: Vec<JValRef>) -> JResult {
        self.state.jlambda(Rc::clone(&self.globals), params, body)
    }
    /// Construct a `jibi` `macro`.
    pub fn jmacro(&mut self, params: Vec<String>, body: Vec<JValRef>) -> JResult {
        self.state.jmacro(Rc::clone(&self.globals), params, body)
    }
    /// Define a `jibi` builtin procedure.
    pub fn jbuiltin<F>(&mut self, name: String, f: F) -> JValRef
    where
        F: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
    {
        let v = self.state.jbuiltin(name.clone(), Rc::new(f));
        self.def(&name, Rc::clone(&v));
        v
    }
    /// Define a `jibi` builtin special form.
    pub fn jspecialform<F>(&mut self, name: String, f: F) -> JValRef
    where
        F: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
    {
        let v = self.state.jbuiltin(name.clone(), Rc::new(f));
        self.def(&name, Rc::clone(&v));
        v
    }
}
