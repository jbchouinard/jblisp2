[![Rust](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml/badge.svg)](https://github.com/jbchouinard/jblisp2/actions/workflows/rust.yml)
# jbscheme

A tiny interpreted dialect of Scheme, implemented in Rust, for educational purposes.

Inspired by [mit-scheme](https://www.gnu.org/software/mit-scheme/) and
[Build Your Own Lisp](http://www.buildyourownlisp.com/).

It is an educational project, I aim to eventually implement enough features to use
jbscheme for [SICP](https://mitpress.mit.edu/sites/default/files/sicp/index.html),
but it will not be drop-in replacement for mit-scheme since jbscheme is its own dialect,
it is not implementing RnRS.

Features:
- First class functions
- "Procedural" macros
- Lexical scoping and closures
- Reference-counting memory management
- Error handling with `raise` and `try`
- Recursive descent parser

## Test

```bash
make test
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

## Documentation

### Special forms

#### macro

```
(macro <formals> <body>)
```

jbscheme macros are "procedural"; they are simply lambdas which return code.

The body of the macro is first evaluated in the macro's lexical environment.
Then the resulting expression is evaluated in the caller's environment.

Beware of capturing variables from the macro's environment, if you want to refer to
variables in the calling environment, quotes must be used.

This `add-x` macro captures the global binding for `x`:
```
>>> (defmacro add-x (y) (list + x y))
>>> (def x 100)
>>> (add-x 5)
105
>>> (set! x 200)
>>> (add-x 5)
205
>>> ((fn (x) (add-x 5)) 1000)
205
```

In this version, `x` is not captured, it is looked up in the calling environment:
```
>>> (defmacro add-x (y) (list + 'x y))
>>> ((fn (x) (add-x 5)) 1000)
1005
```

#### defmacro

```
(defmacro <name> <formals> <body>)
```

Create and bind macro to name. See [macro](#macro).

### Error Handling

jbscheme has exception raising and handling with these forms:

```
(error "error-message")
(raise <error>)
(try <expr> <exception-handler>)
```

The error value is bound to `err` in the invocation environment of the exception
handler.

Example:

```
>>> (defn errored () (begin (raise (error "oh no!")) (print "never evaluated")))
>>> (errored)
Unhandled Error: oh no!
>>> (try (print "no error") (display err))
no error
>>> (try (errored) (display err))
<error Error "oh no!">
```

## Todo?
- Many builtin functions
- Module system
- Tests
- Variadic lambdas
- Interned values
- Mutable data structures: mutcell, vector, table
- Line numbers for errors
- Memory leak check
- Tail call optimization
- Continuations
- Standard library
