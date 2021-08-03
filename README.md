[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# jbscheme

A tiny interpreted dialect of Scheme, implemented in Rust, for educational purposes.

Inspired by [mit-scheme](https://www.gnu.org/software/mit-scheme/) and
[Build Your Own Lisp](http://www.buildyourownlisp.com/).

It is an educational project, I aim to implement enough features to use jbscheme for
[SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html),
but it will not be drop-in replacement for mit-scheme since jbscheme is its own dialect,
it is not implementing RnRS.

Features:
- First class functions and procedural macros
- Lexical scoping and closures
- Reference-counting memory management
- Error handling with `raise` and `try`
- Recursive descent parser

## Run Interpreter

```bash
cargo run
```

## Todo?
- Many builtin functions
- Module system
- Tests
- Interned values
- Mutable data structures: mutcell, vector, table
- Line numbers for errors
- Memory leak check
- Tail call optimization
- Continuations
- Standard library
