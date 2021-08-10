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
    Ok(state.jbool(x == y))
}

fn jbuiltin_eq(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.jbool(Rc::ptr_eq(&x, &y)))
}

fn jbuiltin_not(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x] = get_n_args(args)?;
    match &*x {
        JVal::Bool(b) => Ok(state.jbool(!b)),
        _ => Err(JError::new(TypeError, "expected a bool")),
    }
}

fn jbuiltin_print(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [s] = get_n_args(args)?;
    match &*s {
        JVal::String(s) => {
            println!("{}", s);
            Ok(state.jnil())
        }
        _ => Err(JError::new(TypeError, "expected string")),
    }
}

fn jbuiltin_repr(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x] = get_n_args(args)?;
    Ok(state.jstring(repr(&x)))
}

fn jbuiltin_display_debug(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    println!("{:?}", val);
    Ok(state.jnil())
}

fn jbuiltin_display_debug_pretty(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    println!("{:#?}", val);
    Ok(state.jnil())
}

fn jbuiltin_display_ptr(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    println!("{:p}", val);
    Ok(state.jnil())
}

fn jbuiltin_display_code(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    let (t, params, code) = match &*val {
        JVal::Lambda(jl) => ("fn".to_string(), &jl.params, &jl.code),
        JVal::Macro(jl) => ("fn".to_string(), &jl.params, &jl.code),
        _ => {
            return Err(JError::new(
                TypeError,
                "expected non-builtin lambda or macro",
            ))
        }
    };
    println!(
        "({} {} {})",
        t,
        params,
        code.iter()
            .map(|v| repr(v))
            .collect::<Vec<String>>()
            .join(" ")
    );
    Ok(state.jnil())
}

fn jspecial_def(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match &*jsym {
        JVal::Symbol(s) => s,
        _ => return Err(JError::new(TypeError, "expected a symbol")),
    };
    let jval = eval(jval, Rc::clone(&env), state)?;
    env.define(sym, jval);
    Ok(state.jnil())
}

fn jspecial_set(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match &*jsym {
        JVal::Symbol(s) => s,
        _ => return Err(JError::new(TypeError, "expected a symbol")),
    };
    let jval = eval(jval, Rc::clone(&env), state)?;
    env.set(sym, jval, state)?;
    Ok(state.jnil())
}

fn jspecial_lambda(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let ([pvals], exprs) = get_n_plus_args(args)?;
    let mut params = vec![];
    for val in pvals.iter_list()? {
        match &*val {
            JVal::Symbol(s) => params.push(s.to_string()),
            _ => return Err(JError::new(TypeError, "expected a list of symbols")),
        }
    }
    state.jlambda(env, params, exprs)
}

fn jspecial_macro(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let ([pvals], exprs) = get_n_plus_args(args)?;
    let mut params = vec![];
    for val in pvals.iter_list()? {
        match &*val {
            JVal::Symbol(s) => params.push(s.to_string()),
            _ => return Err(JError::new(TypeError, "expected a list of symbols")),
        }
    }
    state.jmacro(env, params, exprs)
}

fn jspecial_if(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [pred, thencode, elsecode] = get_n_args(args)?;
    let pred = eval(pred, Rc::clone(&env), state)?;
    let pred = match &*pred {
        JVal::Bool(b) => *b,
        _ => return Err(JError::new(TypeError, "expected bool")),
    };
    if pred {
        eval(thencode, env, state)
    } else {
        eval(elsecode, env, state)
    }
}

fn jbuiltin_type(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    Ok(state.jsymbol(
        match *val {
            JVal::Nil => "nil",
            JVal::Pair { .. } => "pair",
            JVal::Quote(_) => "quote",
            JVal::Int(_) => "integer",
            JVal::Bool(_) => "bool",
            JVal::Symbol { .. } => "symbol",
            JVal::String(_) => "string",
            JVal::Error(_) => "error",
            JVal::Lambda(_) => "lambda",
            JVal::Macro(_) => "macro",
            JVal::Builtin(_) => "builtin",
            JVal::SpecialForm(_) => "specialform",
        }
        .to_string(),
    ))
}

fn jbuiltin_eval(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [expr] = get_n_args(args)?;
    eval(expr, env, state)
}

fn jbuiltin_evalfile(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [file] = get_n_args(args)?;
    match &*file {
        JVal::String(s) => match state.eval_file(s, env) {
            Ok(Some(val)) => Ok(val),
            Ok(None) => Ok(state.jnil()),
            Err((pos, je)) => Err(JError::new(EvalError, &format!("{}: {}", pos, je))),
        },
        _ => Err(JError::new(TypeError, "expected string")),
    }
}

fn jbuiltin_begin(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    Ok(args.iter_list().unwrap().last().unwrap())
}

fn jbuiltin_exit(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [exitcode] = get_n_args(args)?;
    match &*exitcode {
        JVal::Int(n) => std::process::exit((*n).try_into().unwrap()),
        _ => Err(JError::new(TypeError, "expected an int")),
    }
}

fn add_builtin<T>(name: &str, f: T, env: &JEnv, state: &mut JState)
where
    T: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
{
    env.define(name, state.jbuiltin(name.to_string(), Rc::new(f)));
}

fn add_special_form<T>(name: &str, f: T, env: &JEnv, state: &mut JState)
where
    T: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
{
    env.define(name, state.jspecialform(name.to_string(), Rc::new(f)));
}

pub fn add_builtins(env: &JEnv, state: &mut JState) {
    // Program flow
    add_builtin("begin", jbuiltin_begin, env, state);
    add_builtin("exit", jbuiltin_exit, env, state);
    add_special_form("if", jspecial_if, env, state);

    // Comparison
    add_builtin("eq?", jbuiltin_eq, env, state);
    add_builtin("equal?", jbuiltin_equal, env, state);

    // Print, debug
    add_builtin("repr", jbuiltin_repr, env, state);
    add_builtin("print", jbuiltin_print, env, state);
    add_builtin("type", jbuiltin_type, env, state);
    add_builtin("dd", jbuiltin_display_debug, env, state);
    add_builtin("ddp", jbuiltin_display_debug_pretty, env, state);
    add_builtin("dda", jbuiltin_display_ptr, env, state);
    add_builtin("ddc", jbuiltin_display_code, env, state);

    // Arithmetic operators
    add_builtin("+", jbuiltin_add, env, state);
    add_builtin("*", jbuiltin_mul, env, state);

    // Logical operators
    add_builtin("not", jbuiltin_not, env, state);

    // List
    add_builtin("list", jbuiltin_list, env, state);
    add_builtin("cons", jbuiltin_cons, env, state);
    add_builtin("car", jbuiltin_car, env, state);
    add_builtin("cdr", jbuiltin_cdr, env, state);
    add_builtin("list?", jbuiltin_is_list, env, state);

    // String
    add_builtin("concat", jbuiltin_concat, env, state);

    // Var, function definition
    add_special_form("def", jspecial_def, env, state);
    add_special_form("set!", jspecial_set, env, state);
    add_special_form("fn", jspecial_lambda, env, state);

    // Error handling
    add_builtin("error", jbuiltin_error, env, state);
    add_builtin("raise", jbuiltin_raise, env, state);
    add_special_form("try", jspecial_try, env, state);

    // Metaprogramming
    add_builtin("eval", jbuiltin_eval, env, state);
    add_builtin("evalfile", jbuiltin_evalfile, env, state);
    add_special_form("quote", jspecial_quote, env, state);
    add_special_form("macro", jspecial_macro, env, state);

    // Sys
    add_builtin("getenv", jbuiltin_getenv, env, state);
}
