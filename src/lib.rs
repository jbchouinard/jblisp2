//! # jbscheme Rust Interop
//!
//! ## Memory
//! All `jbscheme` values are reference-counted with [`Rc`](std::rc::Rc). There is no
//! explicit garbage collection or cycle detection.
//!
//! Each `jbscheme` [`Interpreter`] has its own state and global environment. Multiple
//! interpreters can run in parallel, but `jbscheme` values cannot be shared
//! between threads.
//!
//! `jbscheme` values must be constructed with the methods on [`Interpreter`] or [`JState`].
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
//! [`JError`] represents exceptions in the `jbscheme` language. They can arise
//! from a call to `raise` in `jbscheme` code, or from parsing or evaluation errors.
//!
//! [`JError`] may be found both in an error value ([`JVal::Error`]), representing
//! an `error` created in `jbscheme` but not raised, and in [`Err`]`(`[`JError`]`)`
//! when it is raised.
//!
//! ## Example
//! ```
//! // hello.rs
//! use jbscheme::Interpreter;
//!
//! fn main() {
//!     // Create an interpreter pre-loaded with definitions for builtins; and constants,
//!     // lambdas and macros defined by the prelude.
//!     // (Interpreter::new() instead creates a bare interpreter, with empty globals.)
//!     let mut interpreter = Interpreter::default();
//!     match interpreter.eval_str("hello.rs", r#"(print "Hello World!")"#) {
//!         Ok(Some(jval)) => println!("{}", jval),
//!         Ok(None) => (),
//!         Err(je) => eprintln!("{}", je),
//!     };
//! }
//! ```
mod builtin;
mod env;
mod error;
mod eval;
mod interpreter;
mod primitives;
mod reader;
mod repr;
mod state;

use eval::eval;
use primitives::*;
use repr::repr;

// Exports
pub use env::{JEnv, JEnvRef};
pub use error::{JError, JResult};
pub use interpreter::{Interpreter, PRELUDE};
pub use primitives::{JPair, JVal, JValRef};
pub use reader::parser::Parser;
pub use reader::tokenizer::{Token, TokenError, TokenIter, TokenValidator, TokenValue, Tokenizer};
pub use state::JState;
