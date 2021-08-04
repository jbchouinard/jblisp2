[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# JB Scheme

A tiny interpreted dialect of Scheme, implemented in Rust, for educational purposes.

Inspired by [mit-scheme](https://www.gnu.org/software/mit-scheme/) and
[Build Your Own Lisp](http://www.buildyourownlisp.com/).

jbscheme does not aim to be a fully-featured Scheme language, it is an exercise
in language design and Rust programming. Eventually I would like to at least
implement enough features to use jbscheme to complete most exercises and
projects from [SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html).
However it will not be drop-in replacement for mit-scheme, it is my own
made up dialect, I do not aim to implement RnRS.

Features:
- First class functions
- "Procedural" macros
- Lexical scoping and closures
- Exceptions
- Reference-counted memory management

## Test
```bash
make test
```

To print more details, set the environment variable `TEST_VERBOSE=1`.

## Run
```bash
cargo run
```

## Install
```bash
make
make install
```

## Documentation

See [MANUAL.md](MANUAL.md).

## Todo?
- More builtin functions
- More tests
- More standard library
- Numeric data types: float, complex 
- Variadic lambdas
- First class envs
- Imports, namespaces
- Interned values
- Mutable data structures: mutpair, vector, table
- Tracebacks
- Memory leak check
- Tail call optimization
- Continuations
