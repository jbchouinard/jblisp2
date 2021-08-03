[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# jblisp2

A Lisp dialect implemented in Rust. Not very complete yet.

Features:
- Exception handling with `raise` and `try`
- Reference-counted values
- Hand-written tokenizer and parser

## Run Interpreter

```bash
cargo run
```

## Todo?
- Import, namespaces
- Tail call elimination
- Mutable data structures
- Tests
