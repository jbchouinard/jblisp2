[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# jblisp2

A Lisp dialect implemented in Rust. Not very complete yet.

Features:
- Error handling with `raise` and `try`
- First class functions and procedural macros
- Reference-counting memory management
- Hand-written tokenizer and parser

## Run Interpreter

```bash
cargo run
```

## Todo?
- Add missing builtins
- Module system
- Tests
- Interned values
- Mutable data structures: mutcell, vector, table
- Line numbers for errors
- Memory leak check
- Tail call elimination
- stdlib
