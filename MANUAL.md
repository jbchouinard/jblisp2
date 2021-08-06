# JB Scheme Manual

## Primitive Types
- pair
- string
- symbol
- integer
- bool
- lambda
- macro
- quote
- error
- builtin function
- builtin specialform

## Forms

### Lambdas, Bindings & Control Flow

#### def
```
(def <name> <expr>)
```

Create and assign binding in local scope.

#### set!
```
(set! <name> <expr>)
```
Change existing binding. Raises error if a binding does not already exists.

#### fn
```
(fn <formals> <expr>)
```
Create a lambda (function).

#### defn
```
(defn <name> <formals> <expr> ...)
```
Create lambda and bind it to `name` (with implicit `begin`).

```
>>> (defn increment (x) (+ x 1))
>>> (increment 1)
2
```

#### let
```
(let <name> <value> <expr> ...)
```
Create a binding in a new local scope.

```
>>> (let x 12 (display x))
12
```

#### if
```
(if <pred:bool> <then:expr> <else:expr>)
```
Evaluates only `then` or `else` conditonally on the value of `pred`.

#### begin
```
(begin <expr> ...)
```
Evaluate expressions sequentially and return value of last expression.

#### apply
```
(apply <f:callable> <args:list>)
```

### Comparison

#### eq?
```
(eq? <expr> <expr>)
```
Identity comparison. Check if two values are the same object.

#### equal?
```
(equal? <expr> <expr>)
```
Value comparison. Check if two values are equal.

### Logical Operators

#### not
```
(not <x:bool>)
```

### Pairs & Lists

#### cons
```
(cons <left:expr> <right:expr>)
```
Construct a pair.

#### car
```
(car <pair>)
```

#### cdr
```
(cdr <pair>)

```

#### list
```
(list <expr> ...)
```

#### nil?
```
(nil? <expr>)
```

#### list?
```
(list? <expr>)
```

#### map
```
(map <f> <list>)
```
Applies `f` to each value of a list and return results in list.

```
>>> (map (fn (x) (* 2 x)) (list 1 2 3))
(2 4 6)
```

#### fold
```
(fold <f> <init> <list>)
```
Applies `f` to each values of a list and accumulate results in `init`.

```
>>> (fold + 0 (list 1 2 3))
6
>>> (fold cons () (list 1 2 3))
(3 2 1)
```

### Strings

#### concat
```
(concat <string> ...)
```
Concatenate multiple strings.

```
(concat "foo" "bar" "baz")
"foobarbaz"
```

### Numbers

#### +
```
(+ <integer> ...)
```

#### *
```
(* <integer> ...)
```

### Inspection & Display

#### print
```
(print <string>)
```

#### repr
```
(repr <expr>)
```
Get string representation of a value.

#### display
```
(display <expr>)
```
Print string representation of a value.

#### type
```
(type <expr>)
```
Inspect type of a value.

```
>>> (type "foo")
string
```

#### type?
```
(type? <expr> <type>)
```
Test type of a value.

```
>>> (type? "foo" :string)
true
```

### Evaluation

#### eval
```
(eval <expr>)
```
Evaluate an expression.

```
>>> (def expr (quote (+ 5 5)))
>>> expr
(+ 5 5)
>>> (eval expr)
10
```

#### evalfile
```
(evalfile <filename:string>)
```
Evaluate file in the global environment.

#### quote
```
(quote <expr>)
```
A quoted expression evaluates to the expression.

```
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

### Macros

#### defmacro
```
(defmacro <name> <formals> <expr> ...)
```

jbscheme macros are "procedural"; they are simply lambdas which return code.

The body of the macro is first evaluated in the macro's lexical environment.
Then the resulting expression is evaluated in the caller's environment.

Beware of capturing variables from the macro's environment, if you want to refer to
variables in the calling environment, use quotation.

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
>>> (def x 100)
>>> ((fn (x) (add-x 5)) 1000)
1005
```

#### macro
```
(macro <formals> <expr>)
```

Create macro. See [defmacro](#defmacro).

### Exceptions

jbscheme has exception raising and handling with these forms:

#### error
```
(error <msg:string>)
```

#### raise
```
(raise <error>)
```

#### try
```
(try <body:expr> <catch:expr>)
```
Try evaluating `body`. If an error is raised, evaluate `catch`; the raised error
is bound to `err`.

```
>>> (defn errored () (begin (raise (error "oh no!")) (print "never evaluated")))
>>> (errored)
Unhandled Error: oh no!
>>> (try (print "no error") (print (concat "handled " (repr err))))
no error
>>> (try (errored) (print (concat "handled " (repr err))))
handled #[error Error "oh no!"]
```

#### assert
```
(assert <pred:bool>)
```

### System

#### getenv
```
(getenv <envvar:string>)
```
Get value of environment variable. Raises exception if the variable is not set
or contains non-UTF8 characters.

#### exit
```
(exit <code:integer>)
```
Exit program with a status code.

### Debug

#### dd
```
(dd <expr>)
```
Print Rust struct debug.

#### ddp
```
(ddp <expr>)
```
Pretty print Rust struct debug.

#### dda
```
(dda <expr>)
```
Print pointer address.

#### ddc
```
(ddc <lambda|macro>)
```
Print code of (non-builtin) lambda or macro. 
