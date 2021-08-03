use std::rc::Rc;

use crate::builtin::args::*;
use crate::builtin::error::*;
use crate::builtin::list::*;
use crate::builtin::math::*;
use crate::*;

mod args;
mod error;
mod list;
mod math;

// TODO: BASE: begin eval truthy? set!
// TODO: LIST: list? nil?
// TODO: CONDITIONALS: and or cond

fn jbuiltin_equal(args: &JCell, _env: JEnvRef) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(JValue::Bool(x == y).into_ref())
}

fn jbuiltin_eq(args: &JCell, _env: JEnvRef) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(JValue::Bool(Rc::ptr_eq(&x, &y)).into_ref())
}

fn jbuiltin_not(args: &JCell, _env: JEnvRef) -> JResult {
    let [x] = get_n_args(args)?;
    match &*x {
        JValue::Bool(b) => Ok(JValue::Bool(!b).into_ref()),
        _ => Err(JError::new("TypeError", "expected a bool")),
    }
}

fn jbuiltin_print(args: &JCell, _env: JEnvRef) -> JResult {
    let [s] = get_n_args(args)?;
    match &*s {
        JValue::String(s) => {
            println!("{}", s);
            Ok(JValue::Cell(JCell::Nil).into_ref())
        }
        _ => Err(JError::new("TypeError", "expected string")),
    }
}

fn jbuiltin_repr(args: &JCell, _env: JEnvRef) -> JResult {
    let [x] = get_n_args(args)?;
    Ok(JValue::String(jrepr(&x)).into_ref())
}

fn jspecial_def(args: &JCell, env: JEnvRef) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match &*jsym {
        JValue::Symbol(s) => s,
        _ => return Err(JError::new("TypeError", "expected a symbol")),
    };
    let jval = jeval(jval, Rc::clone(&env))?;
    env.define(sym, jval);
    Ok(JValue::Cell(JCell::Nil).into_ref())
}

fn jspecial_lambda(args: &JCell, env: JEnvRef) -> JResult {
    let [pvals, code] = get_n_args(args)?;
    let pvals = match &*pvals {
        JValue::Cell(c) => c,
        _ => {
            println!("{:#?}", pvals);
            return Err(JError::new("TypeError", "expected a list of symbols"));
        }
    };
    let mut params = vec![];
    for val in pvals.iter()? {
        match &*val {
            JValue::Symbol(s) => params.push(s.to_string()),
            _ => {
                return {
                    println!("{:#?}", pvals);
                    Err(JError::new("TypeError", "expected a list of symbols"))
                }
            }
        }
    }
    Ok(JValue::Lambda(Box::new(JLambda {
        closure: env,
        code: Rc::clone(&code),
        params,
    }))
    .into_ref())
}

fn jspecial_macro(args: &JCell, env: JEnvRef) -> JResult {
    let [pvals, code] = get_n_args(args)?;
    let pvals = match &*pvals {
        JValue::Cell(c) => c,
        _ => {
            println!("{:#?}", pvals);
            return Err(JError::new("TypeError", "expected a list of symbols"));
        }
    };
    let mut params = vec![];
    for val in pvals.iter()? {
        match &*val {
            JValue::Symbol(s) => params.push(s.to_string()),
            _ => {
                return {
                    println!("{:#?}", pvals);
                    Err(JError::new("TypeError", "expected a list of symbols"))
                }
            }
        }
    }
    Ok(JValue::Macro(Box::new(JLambda {
        closure: env,
        code: Rc::clone(&code),
        params,
    }))
    .into_ref())
}

fn jspecial_if(args: &JCell, env: JEnvRef) -> JResult {
    let [pred, thencode, elsecode] = get_n_args(args)?;
    let pred = jeval(pred, Rc::clone(&env))?;
    let pred = match &*pred {
        JValue::Bool(b) => *b,
        _ => return Err(JError::new("TypeError", "expected bool")),
    };
    if pred {
        jeval(thencode, env)
    } else {
        jeval(elsecode, env)
    }
}

fn add_builtin<T>(name: &str, f: T, env: &JEnv)
where
    T: 'static + Fn(&JCell, JEnvRef) -> JResult,
{
    let val = JValue::Builtin(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.define(name, val.into_ref());
}

fn add_special_form<T>(name: &str, f: T, env: &JEnv)
where
    T: 'static + Fn(&JCell, JEnvRef) -> JResult,
{
    let val = JValue::SpecialForm(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.define(name, val.into_ref());
}

pub fn add_builtins(env: &JEnv) {
    env.define("nil", JValue::Cell(JCell::Nil).into_ref());
    add_builtin("eq?", jbuiltin_eq, env);
    add_builtin("equal?", jbuiltin_equal, env);
    add_builtin("not", jbuiltin_not, env);
    add_builtin("repr", jbuiltin_repr, env);
    add_builtin("print", jbuiltin_print, env);
    add_builtin("+", jbuiltin_add, env);
    add_builtin("*", jbuiltin_mul, env);
    add_special_form("try", jspecial_try, env);
    add_builtin("error", jbuiltin_error, env);
    add_builtin("raise", jbuiltin_raise, env);
    add_builtin("list", jbuiltin_list, env);
    add_builtin("cons", jbuiltin_cons, env);
    add_builtin("car", jbuiltin_car, env);
    add_builtin("cdr", jbuiltin_cdr, env);
    add_special_form("quote", jspecial_quote, env);
    add_special_form("def", jspecial_def, env);
    add_special_form("fn", jspecial_lambda, env);
    add_special_form("macro", jspecial_macro, env);
    add_special_form("if", jspecial_if, env);
}
