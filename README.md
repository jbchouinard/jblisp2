[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# Jibi Scheme

A tiny interpreted dialect of Scheme, implemented in Rust, for fun and learning.

Inspired by [mit-scheme](https://www.gnu.org/software/mit-scheme/) and
[Build Your Own Lisp](http://www.buildyourownlisp.com/).

jibi does not aim to be a fully-featured standard (RnRS) Scheme language,
it is an exercise in language design and implementation, Rust, and metaprogramming.
Eventually I would like to implement enough features to use jibi to complete
exercises and projects from [SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html).

It also does not aim to be fast, at least for now. I have tried to implement jibi
language features in jibi where possible since it was more fun, but it makes
everything quite slow.

Features:
- First class functions
- Lexical scoping and closures
- Procedural macros
- Reader macros
- Exception handling
- Namespaced modules
- Automatic reference counting
- Decimal floating point arithmetic

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
- quasiquote
- ModuleImporter, JIBI_PATH
- Lazy values, lazy lists
- User-defined types, operator overloading
- Specific error catch
- Numeric data types: complex, arbitrary precision int
- Mutable data structures: mutpair, vector, table
- Compiled libs
- FFI
- Continuations
- Standard libraries: io, sockets, math
- Debugger
- Reference cycle detection
- Tail call optimization
- JIT compilation

---

Copyright 2021 Jérome Boisvert-Chouinard
