//! [Jibi Scheme Language Documentation](https://jbchouinard.github.io/jibi/)
//!
//! # Jibi Rust Interop
//!
//! ## Memory
//! All `jibi` values are reference-counted with [`Rc`](std::rc::Rc). There is no
//! explicit garbage collection or cycle detection.
//!
//! Each `jibi` [`Interpreter`] has its own state and global environment. Multiple
//! interpreters can run in parallel, but `jibi` values cannot be shared
//! between threads.
//!
//! `jibi` values must be constructed with the methods on [`Interpreter`] or [`JState`].
//! Passing in references to [`JVal`]'s created outside the interpreter, or from
//! a different interpreter running in the same thread, may break language semantics,
//! since some types are interned separately by each interpreter, and correct behavior
//! of `eq?` relies on interning.
//!
//! Sharing `lambda` or `macro` objects will also produce strange results since they carry
//! closures pointing to environments in the interpreter in which they were defined.
//!
//! `int` and `string` values may also be interned but there is no guarantee that
//! `(eq? "somestring" "somestring")` or `(eq? 100 100)` is ever true so sharing them
//! between interpreters should be fine.
//!
//! ## Error Handling
//! [`JError`] represents exceptions in the `jibi` language. They can arise
//! from a call to `raise` in `jibi` code, or from parsing or evaluation errors.
//!
//! [`JError`] may be found both in an error value ([`JVal::Error`]), representing
//! an `error` created in `jibi` but not raised, and in [`Err`]`(`[`JError`]`)`
//! when it is raised.
//!
//! ## Example
//! ```
//! use jibi::Interpreter;
//!
//! // Create an interpreter pre-loaded with definitions for builtins; and constants,
//! // lambdas and macros defined by the prelude.
//! // (Interpreter::new() instead creates a bare interpreter, with empty globals.)
//! let mut jibi = Interpreter::default();
//! jibi.eval_str("hello.rs", r#"
//!     (defn add (x y) (+ x y))
//! "#).unwrap();
//! let args = vec![jibi.int(10), jibi.int(100)];
//! let res = jibi.call("add", args).unwrap();
//! println!("{}", res);
//! ```
mod builtin;
mod env;
mod error;
mod eval;
mod interpreter;
mod reader;
mod repr;
mod state;
mod traceback;
mod types;

use eval::eval;
use repr::repr;
use types::*;

// Exports
pub use env::{JEnv, JEnvRef};
pub(crate) use error::JErrorKind::*;
pub use error::{JError, JErrorKind, JResult};
pub use interpreter::{Interpreter, PRELUDE};
pub use reader::parser::Parser;
pub use reader::tokenizer::{Token, TokenError, TokenIter, TokenValidator, TokenValue, Tokenizer};
pub use reader::PositionTag;
pub use state::JState;
pub use traceback::TracebackFrame;
pub use types::{JPair, JVal, JValRef};

pub type JException = (PositionTag, JError, Vec<TracebackFrame>);
