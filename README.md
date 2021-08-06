[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# JB Scheme

A tiny interpreted dialect of Scheme, implemented in Rust, for educational purposes.

Inspired by [mit-scheme](https://www.gnu.org/software/mit-scheme/) and
[Build Your Own Lisp](http://www.buildyourownlisp.com/).

jbscheme does not aim to be a fully-featured Scheme language, it is an exercise
in language design and implementation, and Rust programming. Eventually I would like to
implement enough features to use jbscheme to complete exercises and
projects from [SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html).
However it will not be drop-in replacement for mit-scheme, it is a different dialect,
I do not aim to implement RnRS.

Features:
- First class functions
- "Procedural" macros
- Lexical scoping and closures
- Exceptions
- Reference-counted memory management

## Test
```bash
cargo test
```

## Run
```bash
cargo run
```

## Install
```bash
make
make install
```

## Language Documentation

See [MANUAL.md](MANUAL.md).

## Rust Interop Documentation

```bash
cargo doc --open
```

## Todo?
- Comments
- Tracebacks
- More builtin functions
- Tail call optimization
- More tests
- First class envs
- Imports, namespaces
- Mutable data structures: mutpair, vector, table
- Debugger
- Continuations
- Memory leak check, cycle detection
- Numeric data types: float, complex 
- Standard library
