[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# Jibi Scheme

A tiny interpreted dialect of Scheme, implemented in Rust, for educational purposes.

Inspired by [mit-scheme](https://www.gnu.org/software/mit-scheme/) and
[Build Your Own Lisp](http://www.buildyourownlisp.com/).

jibi does not aim to be a fully-featured standard (RnRS) Scheme language,
it is an exercise in language design and implementation, and Rust programming.
Eventually I would like to implement enough features to use jibi to complete
exercises and projects from [SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html).

Features:
- First class functions
- Lexical scoping and closures
- Procedural macros
- Exception handling
- Namespaced modules
- Automatic reference counting

## Documentation
- [Crate Docs](https://jbchouinard.github.io/jibi/crate/jibi/index.html)
- [Language Reference](https://jbchouinard.github.io/jibi/index.html)
  ([PDF](https://jbchouinard.github.io/jibi/Jibi%20Scheme%20Manual.pdf))

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

## Todo?
- JIBI_PATH for imports
- Builtins: lets, or, and, cond
- Reader macro definition
- Specific error catch
- Numeric data types: float, complex, arbitrary precision int
- Mutable data structures: mutpair, vector, table
- Standard libraries: io, sockets, math
- Debugger
- Continuations
- Reference cycle detection
- Tail call optimization
- JIT compilation

---

Copyright 2021 Jérome Boisvert-Chouinard
