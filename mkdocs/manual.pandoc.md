# Jibi Scheme
**version 0.1.0**

A homebrew interpreted, non-RnRS compliant dialect of Scheme.

\newpage
## Types

### Primitive Types

#### string
```nohighlight
"some-string"
```
String are immutable.

*Evaluation Rule:*
A `string` value evaluates to itself.

---

#### symbol
```nohighlight
some-symbol
```
All `symbol` values are interned, therefore `(eq? 'some-symbol 'some-symbol)` is true.

*Evaluation Rule:*
`symbol` values are variable names. When evaluated, a `symbol` is replaced by the value
of its binding in the nearest enclosing scope where it is defined.
An error is raised if `symbol` is not bound in any enclosing scope.

---

#### integer
```nohighlight
100
```
The underlying type for `integer` is `i128`. Integer overflow terminates the program.

*Evaluation Rule:*
An `integer` value evaluates to itself.

---

#### bool
```nohighlight
true
false
```
Only `bool` have truth values, therefore they are the only type that can be used
as predicates, e.g. for `if`.

*Evaluation Rule:*
A `bool` value evaluates to itself.

---

#### nil
```nohighlight
nil
```
In Jibi Scheme, `nil` and all empty lists `()` are the same object, therefore
`(eq? () ())` is true.

*Evaluation Rule:*
`nil` evaluates to itself.

---

### Composite Types

#### pair
```nohighlight
(cons :expr :expr)
```
The `pair`, also known as cons cell, is the basic Scheme compound data type.
It is simply a grouping of two values of any types (2-tuple); the first and second
values are sometimes referred to respectively as the `car` and `cdr`.

*Evaluation Rule:*
`pair` values are evaluated by procedure application, however, only `pair` values
which are `list`'s can be properly applied; evaluating a non-list `pair` raises an error.

---

#### list
```nohighlight
; code
(:callable :expr ...)
; data
()
(cons :expr (cons ()))
(list :expr ...)
```
A `list` value is either the empty list `()`,  or ordered `pair`'s terminated by `()`,
where the `car` of the `pair` is an element of the list, and the `cdr` is the rest of
the list.

Scheme data and code are both represented as lists, which makes meta-programming
easy and fun. See [Quoting and Evaluation](#quoting-and-evaluation)
and [Macro Definition](#macro-definition).

*Evaluation Rule:*
The first value of the list is applied (called) with the rest of the list as arguments.
If the first value of the list is not [`callable`](#callable-types), an error is raised.
Exception: the empty list `()` is not applied, it evaluates to itself.
See [Function Definition](#function-definition).

---

### Special Types

#### quote
```nohighlight
(quote :expr)
'expr
```
Any expression can be quoted, using either the `quote` form or a starting apostrophe `'`.

*Evaluation Rule:*
A quoted expression evaluates to the expression. This is useful to prevent `symbol`
binding and procedure application. See [Quoting and Evaluation](#quoting-and-evaluation).

---

#### error
```nohighlight
(error type "some-message")
```
Error values do no inherently do anything, until they are [`raise`](#raise)'d as exceptions.
See [Exceptions](#exceptions).

*Evaluation Rule:*
An `error` value evaluates to itself.

---

### Callable Types

#### lambda
```nohighlight
(fn params :expr ...)
```

See [Function Definition](#function-definition).

*Evaluation Rule:*
A `lambda` value evaluates to itself. It is applied when it is the first element of a `list`.

---

#### procmacro
```nohighlight
(macro params :expr ...)
```

Procedural macros. See [Macro Definition](#macro-definition).

*Evaluation Rule:*
A `procmacro` value evaluates to itself. It is applied when it is the first element of a `list`.

---

### Builtin Callable Types

#### function
```nohighlight
; not constructable
```
Opaque type containing a builtin function.

*Evaluation Rule:*
A `function` value evaluates to itself. It is applied when it is the first element of a `list`.

---

#### specialform
```nohighlight
; not constructable
```
Opaque type containing a builtin macro.

*Evaluation Rule:*
A `specialform` value evaluates to itself. It is applied when it is the first element of a `list`.

---

\newpage
## Forms

### Binding and Assignment

#### def
```nohighlight
(def name :expr)
```
Create and assign binding in local scope.

---

#### set!
```nohighlight
(set! name :expr)
```
Change existing binding. Raises error if a binding does not already exists.

---

#### let
```nohighlight
(let name value:expr :expr ...)
```
Create a binding in a new local scope.

```nohighlight
; Example
>>> (let x 12 (display x))
12
```

---

#### lets
```nohighlight
(lets ((name value:expr) ...) :expr ...)
```
Create multiple bindings in a new local scope.

```nohighlight
; Example
>>> (lets ((x 5) (y 7))
...    (display x)
...    (display y))
5
7
```

---

#### defglobal
```nohighlight
(defglobal name :expr)
```
Create and assign binding in global env.

---

#### setglobal!
```nohighlight
(setglobal! name :expr)
```
Change existing binding in global env. Raises error if a binding does not already exists.

---

\newpage
### Function Definition

#### defn
```nohighlight
(defn name parameters :expr ...)
```
Create lambda function and bind it to `name`.

Variadic lambdas can be defined with formal parameters like `(x . xs)` - there must
be a single parameter after `.`, which will be a list containing zero or more
arguments depending on the number of arguments passed.

```nohighlight
; Example
>>> (defn increment (x) (+ x 1))
>>> (increment 1)
2
>>> (defn variadic (x y . rest) rest)
>>> (variadic 1)
Unhandled ApplyError "expected at least 2 argument(s)"
>>> (variadic 1 2)
()
>>> (variadic 1 2 3 4)
(3 4)
```

---

#### fn
```nohighlight
(fn parameters :expr ...)
```
Create a lambda (function). See [`defn`](#defn).

---

\newpage
### Control Flow

#### if
```nohighlight
(if predicate:bool then:expr else:expr)
```
Evaluates only `then` or `else` conditonally on the value of `predicate`.

---

#### begin
```nohighlight
(begin :expr ...)
```
Evaluate expressions sequentially and return value of last expression.

---

\newpage
### Comparison

#### eq?
```nohighlight
(eq? :expr :expr)
```
Identity comparison. Check if two values are the same object.

---

#### equal?
```nohighlight
(equal? :expr :expr)
```
Value comparison. Check if two values are equal.

---

\newpage
### Logical Operators

#### not
```nohighlight
(not :bool)
```

---

\newpage
### Pair and List Operations

#### cons
```nohighlight
(cons left:expr right:expr)
```
Construct a pair.

---

#### car
```nohighlight
(car :pair)
```
Get first item of a pair (head of list).

---

#### cdr
```nohighlight
(cdr :pair)

```
Get second item of a pair (rest of list).

---

#### list
```nohighlight
(list :expr ...)
```
Construct a list, which is a linked list made from pairs and termninated by `nil`.

```nohighlight
; Example
>>> (equal? (list 1 2 3) (cons 1 (cons 2 (cons 3 nil))))
true
>>> (equal? (list 1 2 3) (cons 1 (list 2 3)))
```

---

#### empty?
```nohighlight
(empty? :expr)
```
Check if value is the empty list (nil).

---

#### list?
```nohighlight
(list? :expr)
```
Check if value is a nil-terminated list of ordered pairs.

---

#### map
```nohighlight
(map f:procedure vals:list)
```
Applies `f` to each value in a list and return results in list.

```nohighlight
; Example
>>> (map (fn (x) (* 2 x)) (list 1 2 3))
(2 4 6)
```

---

#### fold
```nohighlight
(fold f:procedure init:expr vals:list)
```
Applies `f` to each value in a list and accumulate results in `init`.

```nohighlight
; Example
>>> (fold + 0 (list 1 2 3))
6
>>> (fold cons () (list 1 2 3))
(3 2 1)
```

---

### range
```nohighlight
(range from:integer to:integer)
```
Produce list of integers for range [`from`, `to`), where `to` >= `from`.

---

\newpage
### String Operations

#### concat
```nohighlight
(concat :string ...)
```
Concatenate multiple strings.

```nohighlight
; Example
>>> (concat "foo" "bar" "baz")
"foobarbaz"
```

---

\newpage
### Integer Operations

#### +
```nohighlight
(+ :integer ...)
```

#### -
```nohighlight
(- :integer ...)
```

#### \*
```nohighlight
(* :integer ...)
```

#### /
```nohighlight
(/ :integer ...)
```

---

\newpage
### Printing

#### print
```nohighlight
(print :string)
```

---

#### repr
```nohighlight
(repr :expr)
```
Get string representation of a value.

---

#### display
```nohighlight
(display :expr)
```
Print string representation of a value.

---

\newpage
### Modules

(Work in progress.)

`jibi` has a basic namespaced module system. A module is simply a `.jibi` file.

They provide no privacy, all variables defined in the module scope are accessible
to importers.

Module files are only evaluated once, re-importing gets a reference to the existing
module.

At the moment only the current working directory is searched to find modules,
thus importing "stl/unittest" looks for the file `./stl/unittest.jibi'.`

#### import
```nohighlight
(import module:string as name)
```
Import module and bind it to `name`. 

```nohighlight
; Example
>>> (import "stl/math" as math)
>>> (math::product (list 2 3 4))
24
```

---

#### use
```nohighlight
(use module:symbol name ...)
```
Bind a name from a module into the global scope.

```nohighlight
; Example
>>> (import "stl/math" as math)
>>> (use math product sum)
>>> (product (list 2 3 4))
24
```

---

#### import-from
```nohighlight
(import-from module:string name ...)
```
Import specific names from a module.

```nohighlight
; Example
>>> (import-from "stl/math" product sum)
>>> (sum (list 2 3 4))
9
>>> (product (list 2 3 4))
24
```

---

\newpage
### Type Inspection

#### type
```nohighlight
(type :expr)
```
Inspect type of a value.

```nohighlight
; Example
>>> (type "foo")
string
```

---

#### type?
```nohighlight
(type? :expr type)
(string? :expr)
(symbol? :expr)
...
```
Test type of a value. There are also convenience functions for every type.

```nohighlight
; Example
>>> (type? "foo" string)
true
>>> (integer? "foo")
false
```

---

\newpage
### Quoting and Evaluation

#### quote
```nohighlight
(quote :expr)
```
A quoted expression evaluates to the expression.

```nohighlight
; Example
>>> (def a 100)
>>> a
100
>>> (quote a)
a
>>> (+ 5 5)
10
>>> (quote (+ 5 5))
(+ 5 5)
```

---

#### eval
```nohighlight
(eval :expr)
```
Evaluate an expression.

```nohighlight
; Example
>>> (def expr (quote (+ 5 5)))
>>> expr
(+ 5 5)
>>> (eval expr)
10
```

---

#### evalfile
```nohighlight
(evalfile filename:string)
```
Evaluate file in the global environment.

---

#### apply
```nohighlight
(apply :procedure :list)
```
Apply a procedure to a list of arguments.

```nohighlight
; Example
>>> (apply + (list 1 2 3))
6
```

---

\newpage
### Macro Definition

#### defmacro
```nohighlight
(defmacro name formals :expr ...)
```
jibi macros are "procedural"; they are simply lambdas which return code.

The body of the macro is first evaluated in the macro's lexical environment.
Then the resulting expression is evaluated in the caller's environment.

Beware of capturing variables from the macro's environment; if you want to refer to
variables in the invocation environment, use quotation.

This `add-x` macro captures the global binding for `x`:
```nohighlight
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

In this version, `x` is not captured; the value of `x` is taken from the local scope
where the macro is called:
```nohighlight
>>> (def x 100)
>>> (defmacro add-x (y) (list + 'x y))
>>> ((fn (x) (add-x 5)) 1000)
1005
```

---

#### macro
```nohighlight
(macro formals :expr ...)
```
Create macro. See ['defmacro'](#defmacro).

---

\newpage
### Exceptions

Errors can be raised to interrupt program flow, and can be caught with the `try` form.

#### error
```nohighlight
(error type:symbol reason:string)
```
Create error with custom type.

#### exception
```nohighlight
(exception reason:string)
```
Create error of type Exception.

#### raise
```nohighlight
(raise :error)
```
Raise an error (can be any error type, not just Exception).

#### try
```nohighlight
(try body:expr catch:expr)
```

Try evaluating `body`. If an error is raised, evaluate `catch`; the raised error value
is bound to `err` when `catch` is evaluated.

```nohighlight
; Example
>>> (defn errored ()
...		(raise (exception "oh no!"))
...		(print "never evaluated"))
>>> (errored)
Unhandled Error: oh no!
>>> (try (print "no error") (print (concat "handled " (repr err))))
no error
>>> (try (errored) (print (concat "handled " (repr err))))
handled #[error Exception "oh no!"]
```

---

#### assert
```nohighlight
(assert predicate:bool)
```
Raises an exception if `predicate` is false.

---

\newpage
### Environment Procedures

#### env
```nohighlight
(env)
```
Get the nearest enclosing environment (most local scope).

---

#### env-lookup
```nohighlight
(env-lookup :env :symbol)
```
Look up symbol in the given environment.

---

#### env-def
```nohighlight
(env-def :env :symbol :expr)
```
Define symbol in the given environment.

---

#### env-set
```nohighlight
(env-set! :env :symbol :expr)
```
Set symbol in the given environment.

---

#### env-parent
```nohighlight
(env-parent :env)
```
Get parent env, or `nil` if there is no parent env.

---

#### env-globals
```nohighlight
(env-globals)
```
Get the global environment.

---

\newpage
### System Procedures

#### environment-variable
```nohighlight
(environment-variable var:string)
```
Get value of environment variable. Raises exception if the variable is not set
or contains non-UTF8 characters.

---

#### exit
```nohighlight
(exit :integer)
```
Exit program with a status code.

---

\newpage
### Debugging

#### dd
```nohighlight
(dd :expr)
```
Print Rust struct debug.

---

#### ddp
```nohighlight
(ddp :expr)
```
Pretty print Rust struct debug.

---

#### dda
```nohighlight
(dda :expr)
```
Print pointer address.

---

#### ddc
```nohighlight
(ddc :lambda|:procmacro)
```
Print code of (non-builtin) lambda or macro. 

---

#### ddm
```nohighlight
(ddm :procmacro :expr ...)
```
Print code generated by a `procmacro` for the given arguments.

---

\newpage
## Standard Libraries

### unittest

#### test

#### test-start

#### test-exit

#### test-suite

#### assert-not

#### assert-eq

#### assert-equal

#### assert-raise
