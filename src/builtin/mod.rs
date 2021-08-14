use std::rc::Rc;

use crate::*;
use args::*;
use debug::*;
use env::*;
use error::*;
use list::*;
use math::*;
use readermacro::*;
use string::*;
use sys::*;

mod args;
mod debug;
mod env;
mod error;
mod list;
mod math;
mod readermacro;
mod string;
mod sys;

// TODO: loop, and, or, cond

fn jbuiltin_equal(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.bool(x == y))
}

fn jbuiltin_eq(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.bool(Rc::ptr_eq(&x, &y)))
}

fn jbuiltin_not(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x] = get_n_args(args)?;
    let x = x.to_bool()?;
    Ok(state.bool(!x))
}

fn jspecial_and(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    let x = eval(x, Rc::clone(&env), state)?.to_bool()?;
    if !x {
        return Ok(state.bool(false));
    }
    let y = eval(y, env, state)?.to_bool()?;
    Ok(state.bool(y))
}

fn jspecial_or(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    let x = eval(x, Rc::clone(&env), state)?.to_bool()?;
    if x {
        return Ok(state.bool(true));
    }
    let y = eval(y, env, state)?.to_bool()?;
    Ok(state.bool(y))
}

fn jbuiltin_print(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [s] = get_n_args(args)?;
    let s = s.to_str()?;
    println!("{}", s);
    Ok(state.nil())
}

fn jbuiltin_repr(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x] = get_n_args(args)?;
    Ok(state.string(repr(&x)))
}

fn jspecial_def(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [sym, val] = get_n_args(args)?;
    let sym = sym.to_symbol()?;
    let val = eval(val, Rc::clone(&env), state)?;
    env.define(sym, val);
    Ok(state.nil())
}

fn jspecial_set(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [sym, val] = get_n_args(args)?;
    let sym = sym.to_symbol()?;
    let val = eval(val, Rc::clone(&env), state)?;
    env.set(sym, val, state)?;
    Ok(state.nil())
}

fn jspecial_lambda(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let ([pvals], exprs) = get_n_plus_args(args)?;
    let mut params = vec![];
    for val in pvals.iter_list()? {
        params.push(val.to_symbol()?.to_owned())
    }
    state.lambda(env, params, exprs, None)
}

fn jspecial_named_lambda(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let ([name, pvals], exprs) = get_n_plus_args(args)?;
    let name = name.to_str()?;
    let mut params = vec![];
    for val in pvals.iter_list()? {
        params.push(val.to_symbol()?.to_owned())
    }
    state.lambda(env, params, exprs, Some(name.to_string()))
}

fn jspecial_macro(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let ([pvals], exprs) = get_n_plus_args(args)?;
    let mut params = vec![];
    for val in pvals.iter_list()? {
        params.push(val.to_symbol()?.to_owned())
    }
    state.r#macro(env, params, exprs, None)
}

fn jspecial_named_macro(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let ([name, pvals], exprs) = get_n_plus_args(args)?;
    let name = name.to_str()?;
    let mut params = vec![];
    for val in pvals.iter_list()? {
        params.push(val.to_symbol()?.to_owned())
    }
    state.r#macro(env, params, exprs, Some(name.to_string()))
}

fn jspecial_cond(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let ([], clauses) = get_n_plus_args(args)?;
    let mut conds: Vec<(JValRef, Vec<JValRef>)> = vec![];
    // Parse all clauses first to error out immediately if the cond form is ill-formed
    for clause in clauses {
        let clause = clause.to_pair()?;
        let pred = clause.car();
        let exprs: Vec<JValRef> = clause.cdr().iter_list()?.collect();
        conds.push((pred, exprs));
    }
    for (pred, exprs) in conds {
        let pred = eval(pred, Rc::clone(&env), state)?.to_bool()?;
        if pred {
            let mut last_res = state.nil();
            for expr in exprs {
                last_res = eval(expr, Rc::clone(&env), state)?;
            }
            return Ok(last_res);
        }
    }
    // In case there are zero clauses
    Ok(state.nil())
}

fn jbuiltin_type(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    Ok(state.symbol(
        match *val {
            JVal::Nil => "nil",
            JVal::Pair { .. } => "pair",
            JVal::Quote(_) => "quote",
            JVal::Int(_) => "integer",
            JVal::Float(_) => "float",
            JVal::Bool(_) => "bool",
            JVal::Symbol { .. } => "symbol",
            JVal::String(_) => "string",
            JVal::Error(_) => "error",
            JVal::Lambda(_) => "lambda",
            JVal::Macro(_) => "macro",
            JVal::Builtin(_) => "builtin",
            JVal::SpecialForm(_) => "specialform",
            JVal::Env(_) => "env",
            JVal::Token(_) => "token",
            JVal::TokenMatcher(_) => "tokenmatcher",
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
    let file = file.to_str()?;
    match state.eval_file(file, env) {
        Ok(Some(val)) => Ok(val),
        Ok(None) => Ok(state.nil()),
        Err((pos, je, _)) => Err(JError::new(EvalError, &format!("{}: {}", pos, je))),
    }
}

fn jspecial_import(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    // (import "foo/bar" as bar)
    let ([file], rest) = get_n_plus_args(args)?;
    if rest.len() != 2 {
        return Err(JError::new(EvalError, "invalid import form"));
    }
    let sym_as = rest[0].to_symbol()?;
    if sym_as != "as" {
        return Err(JError::new(EvalError, "invalid import form"));
    }
    let file = format!("{}.jibi", eval(file, Rc::clone(&env), state)?.to_str()?);
    let name = rest[1].to_symbol()?;
    let module = state.import_module(file, Rc::clone(&env))?;
    env.define(name, module);
    Ok(state.nil())
}

fn jbuiltin_begin(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    Ok(args.iter_list().unwrap().last().unwrap())
}

fn add_builtin<T>(name: &str, f: T, env: &JEnv, state: &mut JState)
where
    T: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
{
    env.define(name, state.builtin(name.to_string(), Rc::new(f)));
}

fn add_special_form<T>(name: &str, f: T, env: &JEnv, state: &mut JState)
where
    T: 'static + Fn(JValRef, JEnvRef, &mut JState) -> JResult,
{
    env.define(name, state.specialform(name.to_string(), Rc::new(f)));
}

pub fn add_builtins(env: &JEnv, state: &mut JState) {
    // Constants
    env.define("INTMIN", state.int(JTInt::MIN));
    env.define("INTMAX", state.int(JTInt::MAX));

    // Program flow
    add_builtin("begin", jbuiltin_begin, env, state);
    add_special_form("cond", jspecial_cond, env, state);

    // Comparison
    add_builtin("eq?", jbuiltin_eq, env, state);
    add_builtin("equal?", jbuiltin_equal, env, state);

    // Print
    add_builtin("repr", jbuiltin_repr, env, state);
    add_builtin("print", jbuiltin_print, env, state);
    add_builtin("type", jbuiltin_type, env, state);

    // Number procedures
    add_builtin("+", jbuiltin_add, env, state);
    add_builtin("-", jbuiltin_sub, env, state);
    add_builtin("*", jbuiltin_mul, env, state);
    add_builtin("/", jbuiltin_div, env, state);
    add_builtin("=", jbuiltin_num_eq, env, state);
    add_builtin("<", jbuiltin_lt, env, state);
    add_builtin("<=", jbuiltin_lte, env, state);
    add_builtin(">", jbuiltin_gt, env, state);
    add_builtin(">=", jbuiltin_gte, env, state);
    add_builtin("as-float", jbuiltin_as_float, env, state);
    add_builtin("as-integer", jbuiltin_as_int, env, state);

    // Logical operators
    add_builtin("not", jbuiltin_not, env, state);
    add_special_form("or", jspecial_or, env, state);
    add_special_form("and", jspecial_and, env, state);

    // List
    add_builtin("list", jbuiltin_list, env, state);
    add_builtin("cons", jbuiltin_cons, env, state);
    add_builtin("car", jbuiltin_car, env, state);
    add_builtin("cdr", jbuiltin_cdr, env, state);
    add_builtin("list?", jbuiltin_is_list, env, state);

    // String
    add_builtin("len", jbuiltin_len, env, state);
    add_builtin("concat", jbuiltin_concat, env, state);
    add_builtin("contains?", jbuiltin_contains, env, state);
    add_builtin("split", jbuiltin_split, env, state);
    add_builtin("substring", jbuiltin_substring, env, state);
    add_builtin("replace", jbuiltin_replace, env, state);
    add_builtin("parse-integer", jbuiltin_parse_int, env, state);
    add_builtin("parse-float", jbuiltin_parse_float, env, state);

    // Var, function definition
    add_special_form("def", jspecial_def, env, state);
    add_special_form("set!", jspecial_set, env, state);
    add_special_form("fn", jspecial_lambda, env, state);
    add_special_form("nfn", jspecial_named_lambda, env, state);

    // Exceptions
    add_builtin("error", jbuiltin_error, env, state);
    add_builtin("exception", jbuiltin_exception, env, state);
    add_builtin("raise", jbuiltin_raise, env, state);
    add_special_form("try", jspecial_try, env, state);

    // Modules
    add_special_form("import", jspecial_import, env, state);

    // Metaprogramming
    add_builtin("eval", jbuiltin_eval, env, state);
    add_builtin("evalfile", jbuiltin_evalfile, env, state);
    add_special_form("quote", jspecial_quote, env, state);
    add_special_form("macro", jspecial_macro, env, state);
    add_special_form("nmacro", jspecial_named_macro, env, state);

    // Env
    add_builtin("env", jbuiltin_env, env, state);
    add_builtin("env-lookup", jbuiltin_env_lookup, env, state);
    add_builtin("env-def", jbuiltin_env_def, env, state);
    add_builtin("env-set!", jbuiltin_env_set, env, state);
    add_builtin("env-parent", jbuiltin_env_parent, env, state);

    // Sys
    add_builtin("getenv", jbuiltin_get_env_var, env, state);
    add_builtin("exit", jbuiltin_exit, env, state);

    // Debug
    add_builtin("dd", jbuiltin_display_debug, env, state);
    add_builtin("ddp", jbuiltin_display_debug_pretty, env, state);
    add_builtin("dda", jbuiltin_display_ptr, env, state);
    add_builtin("ddc", jbuiltin_display_code, env, state);
    add_special_form("ddm", jspecial_display_debug_macro, env, state);

    // Reader macros
    add_builtin("token", jbuiltin_token, env, state);
    add_builtin("token-match", jbuiltin_tokenmatcher, env, state);
    add_builtin("token-value", jbuiltin_token_value, env, state);
    add_builtin("token-type", jbuiltin_token_type, env, state);
    add_builtin("reader-macro!", jbuiltin_install_reader_macro, env, state);
}
