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

// TODO: BASE: begin eval apply quote truthy?
// TODO: LIST: list list? nil?
// TODO: CONDITIONALS: and or if cond

fn jbuiltin_equal(args: Args, _env: JEnvRef) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(JValue::Bool(x == y).into_ref())
}

fn jbuiltin_eq(args: Args, _env: JEnvRef) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(JValue::Bool(Rc::ptr_eq(&x, &y)).into_ref())
}

fn jbuiltin_not(args: Args, _env: JEnvRef) -> JResult {
    let [x] = get_n_args(args)?;
    match &*x {
        JValue::Bool(b) => Ok(JValue::Bool(!b).into_ref()),
        _ => Err(JError::new("TypeError", "expected a bool")),
    }
}

fn jbuiltin_print(args: Args, _env: JEnvRef) -> JResult {
    let [s] = get_n_args(args)?;
    match &*s {
        JValue::String(s) => {
            println!("{}", s);
            Ok(JValue::Cell(JCell::Nil).into_ref())
        }
        _ => Err(JError::new("TypeError", "expected string")),
    }
}

fn jbuiltin_repr(args: Args, _env: JEnvRef) -> JResult {
    let [x] = get_n_args(args)?;
    Ok(JValue::String(jrepr(x)).into_ref())
}

fn jbuiltin_def(args: Args, env: JEnvRef) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match &*jsym {
        JValue::Symbol(s) => s,
        _ => return Err(JError::new("TypeError", "expected a symbol")),
    };
    let jval = jeval(jval, Rc::clone(&env))?;
    env.define(sym, jval);
    Ok(JValue::Cell(JCell::Nil).into_ref())
}

fn jbuiltin_lambda(args: Args, env: JEnvRef) -> JResult {
    let [pvals, code] = get_n_args(args)?;
    let pvals = match &*pvals {
        JValue::Cell(c) => c,
        _ => return Err(JError::new("TypeError", "expected a list of symbols")),
    };
    let mut params = vec![];
    for val in pvals.iter()? {
        match &*val {
            JValue::Symbol(s) => params.push(s.to_string()),
            _ => return Err(JError::new("TypeError", "expected a list of symbols")),
        }
    }
    Ok(JValue::Lambda(Box::new(JLambda {
        closure: env,
        code: Rc::clone(&code),
        params,
    }))
    .into_ref())
}

fn add_builtin<T>(name: &str, f: T, env: &JEnv)
where
    T: 'static + Fn(Args, JEnvRef) -> JResult,
{
    let val = JValue::Builtin(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.define(name, val.into_ref());
}

fn add_builtin_macro<T>(name: &str, f: T, env: &JEnv)
where
    T: 'static + Fn(Args, JEnvRef) -> JResult,
{
    let val = JValue::BuiltinMacro(JBuiltin {
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
    add_builtin("error", jbuiltin_error, env);
    add_builtin("raise", jbuiltin_raise, env);
    add_builtin("cons", jbuiltin_cons, env);
    add_builtin("car", jbuiltin_car, env);
    add_builtin("cdr", jbuiltin_cdr, env);
    add_builtin_macro("try", jbuiltin_try, env);
    add_builtin_macro("def", jbuiltin_def, env);
    add_builtin_macro("fn", jbuiltin_lambda, env);
}
