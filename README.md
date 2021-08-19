[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# Jibi Scheme

A small tree-walk interpreter for a homebrew dialect of Scheme, implemented in Rust,
for fun and learning.

Inspired by [Build Your Own Lisp](http://www.buildyourownlisp.com/).

jibi does not aim to be a fully-featured standard (RnRS) Scheme language,
Eventually I would like to implement enough features to use jibi to complete
exercises and projects from [SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html).

Since it is a tree-walk interpreter it is horribly slow, I would to like to
re-implement a future version to run on a bytecode VM instead.

Features:
- First class functions
- Lexical scoping and closures
- Procedural macros
- Reader macros
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

User install:

```bash
PREFIX=~/.local make build install
```

System install:

```bash
make build
sudo make install
```

## Todo
- Documentation: quasiquote, vectors

---

Copyright 2021 Jérome Boisvert-Chouinard
