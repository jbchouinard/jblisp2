use std::convert::TryInto;
use std::rc::Rc;

use crate::builtin::args::*;
use crate::builtin::error::*;
use crate::builtin::list::*;
use crate::builtin::math::*;
use crate::builtin::string::*;
use crate::builtin::sys::*;
use crate::*;

mod args;
mod error;
mod list;
mod math;
mod string;
mod sys;

// TODO: loop, map
// TODO: CONDITIONALS: and or cond

fn jbuiltin_equal(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(JVal::bool(x == y, state))
}

fn jbuiltin_eq(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(JVal::bool(Rc::ptr_eq(&x, &y), state))
}

fn jbuiltin_not(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x] = get_n_args(args)?;
    match &*x {
        JVal::Bool(b) => Ok(JVal::bool(!b, state)),
        _ => Err(JError::new("TypeError", "expected a bool")),
    }
}

fn jbuiltin_print(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [s] = get_n_args(args)?;
    match &*s {
        JVal::String(s) => {
            println!("{}", s);
            Ok(JVal::nil())
        }
        _ => Err(JError::new("TypeError", "expected string")),
    }
}

fn jbuiltin_repr(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x] = get_n_args(args)?;
    Ok(JVal::str(jrepr(&x), state))
}

fn jspecial_def(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match &*jsym {
        JVal::Symbol(s) => s,
        _ => return Err(JError::new("TypeError", "expected a symbol")),
    };
    let jval = jeval(jval, Rc::clone(&env), state)?;
    env.define(sym, jval);
    Ok(JVal::nil())
}

fn jspecial_set(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match &*jsym {
        JVal::Symbol(s) => s,
        _ => return Err(JError::new("TypeError", "expected a symbol")),
    };
    let jval = jeval(jval, Rc::clone(&env), state)?;
    env.set(sym, jval)?;
    Ok(JVal::nil())
}

fn jspecial_lambda(args: JValRef, env: JEnvRef, _state: &mut JState) -> JResult {
    let [pvals, code] = get_n_args(args)?;
    let mut params = vec![];
    for val in pvals.iter_list()? {
        match &*val {
            JVal::Symbol(s) => params.push(s.to_string()),
            _ => return Err(JError::new("TypeError", "expected a list of symbols")),
        }
    }
    Ok(JVal::lambda(env, params, Rc::clone(&code)))
}

fn jspecial_macro(args: JValRef, env: JEnvRef, _state: &mut JState) -> JResult {
    let [pvals, code] = get_n_args(args)?;
    let mut params = vec![];
    for val in pvals.iter_list()? {
        match &*val {
            JVal::Symbol(s) => params.push(s.to_string()),
            _ => return Err(JError::new("TypeError", "expected a list of symbols")),
        }
    }
    Ok(JVal::lmacro(env, params, Rc::clone(&code)))
}

fn jspecial_if(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [pred, thencode, elsecode] = get_n_args(args)?;
    let pred = jeval(pred, Rc::clone(&env), state)?;
    let pred = match &*pred {
        JVal::Bool(b) => *b,
        _ => return Err(JError::new("TypeError", "expected bool")),
    };
    if pred {
        jeval(thencode, env, state)
    } else {
        jeval(elsecode, env, state)
    }
}

fn jbuiltin_type(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    Ok(JVal::sym(
        match *val {
            JVal::Nil => "nil",
            JVal::Pair(_) => "pair",
            JVal::Quoted(_) => "quoted",
            JVal::Int(_) => "integer",
            JVal::Bool(_) => "bool",
            JVal::Symbol(_) => "symbol",
            JVal::String(_) => "string",
            JVal::Error(_) => "error",
            JVal::Lambda(_) => "lambda",
            JVal::Macro(_) => "macro",
            JVal::Builtin(_) => "builtin",
            JVal::SpecialForm(_) => "specialform",
        }
        .to_string(),
        state,
    ))
}

fn jbuiltin_eval(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [expr] = get_n_args(args)?;
    jeval(expr, env, state)
}

fn jbuiltin_evalfile(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [file] = get_n_args(args)?;
    match &*file {
        JVal::String(s) => match eval_file(s, env, state)? {
            Some(val) => Ok(val),
            None => Ok(JVal::nil()),
        },
        _ => Err(JError::new("TypeError", "expected string")),
    }
}

fn jbuiltin_begin(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    Ok(args.iter_list().unwrap().last().unwrap())
}

fn jbuiltin_exit(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [exitcode] = get_n_args(args)?;
    match &*exitcode {
        JVal::Int(n) => std::process::exit((*n).try_into().unwrap()),
        _ => Err(JError::new("TypeError", "expected an int")),
    }
}

fn add_builtin<T>(name: &str, f: T, env: &JEnv)
where
    T: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
{
    let val = JVal::Builtin(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.define(name, val.into_ref());
}

fn add_special_form<T>(name: &str, f: T, env: &JEnv)
where
    T: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
{
    let val = JVal::SpecialForm(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.define(name, val.into_ref());
}

pub fn add_builtins(env: &JEnv) {
    // Program flow
    add_builtin("begin", jbuiltin_begin, env);
    add_builtin("exit", jbuiltin_exit, env);
    add_special_form("if", jspecial_if, env);

    // Comparison
    add_builtin("eq?", jbuiltin_eq, env);
    add_builtin("equal?", jbuiltin_equal, env);

    // Print, debug
    add_builtin("repr", jbuiltin_repr, env);
    add_builtin("print", jbuiltin_print, env);
    add_builtin("type", jbuiltin_type, env);

    // Arithmetic operators
    add_builtin("+", jbuiltin_add, env);
    add_builtin("*", jbuiltin_mul, env);

    // Logical operators
    add_builtin("not", jbuiltin_not, env);

    // List
    add_builtin("list", jbuiltin_list, env);
    add_builtin("cons", jbuiltin_cons, env);
    add_builtin("car", jbuiltin_car, env);
    add_builtin("cdr", jbuiltin_cdr, env);
    add_builtin("list?", jbuiltin_is_list, env);

    // String
    add_builtin("concat", jbuiltin_concat, env);

    // Var, function definition
    add_special_form("def", jspecial_def, env);
    add_special_form("set!", jspecial_set, env);
    add_special_form("fn", jspecial_lambda, env);

    // Error handling
    add_builtin("error", jbuiltin_error, env);
    add_builtin("raise", jbuiltin_raise, env);
    add_special_form("try", jspecial_try, env);

    // Metaprogramming
    add_builtin("eval", jbuiltin_eval, env);
    add_builtin("evalfile", jbuiltin_evalfile, env);
    add_special_form("quote", jspecial_quote, env);
    add_special_form("macro", jspecial_macro, env);

    // Sys
    add_builtin("getenv", jbuiltin_getenv, env);
}
