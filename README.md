[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# JB Scheme

A tiny interpreted dialect of Scheme, implemented in Rust, for educational purposes.

Inspired by [mit-scheme](https://www.gnu.org/software/mit-scheme/) and
[Build Your Own Lisp](http://www.buildyourownlisp.com/).

jbscheme does not aim to be a fully-featured standard (RnRS) Scheme language,
it is an exercise in language design and implementation, and Rust programming.
Eventually I would like to implement enough features to use jbscheme to complete
exercises and projects from [SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html).

Features:
- First class functions
- Lexical scoping and closures
- Procedural macros
- Exception handling (raise, try)
- Automatic reference counting

## Documentation

### Language Manual

See [MANUAL.pdf](MANUAL.pdf) or [MANUAL.md](MANUAL.md).

### Rust Docs
```bash
cargo doc --open
```

## Run
```bash
cargo run
```

## Test
```bash
cargo test
```

## Install
```bash
make
make install
```

## Roadmap
- Tracebacks
- More builtin functions
- Tail call optimization
- More tests
- First class envs
- Imports, namespaces
- Numeric data types: float, complex 
- Mutable data structures: mutpair, vector, table
- Debugger
- Continuations
- Reference cycle detection
- Standard library

Copyright 2021 Jérome Boisvert-Chouinard
