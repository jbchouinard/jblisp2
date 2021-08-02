use std::rc::Rc;

use crate::builtin::args::*;
use crate::builtin::error::*;
use crate::builtin::math::*;
use crate::*;

mod args;
mod error;
mod list;
mod math;

// TODO: if, eval, list, head, tail, print, repr, display
// TODO: LIST: list, head, tail
// TODO: CONDITIONALS: and, or, if, cond

fn jbuiltin_equals(args: Args, _env: JEnvRef) -> JResult {
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

fn jbuiltin_define(args: Args, env: JEnvRef) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match &*jsym {
        JValue::Symbol(s) => s,
        _ => return Err(JError::new("TypeError", "expected a symbol")),
    };
    let jval = jeval(jval, Rc::clone(&env))?;
    env.define(sym, jval);
    Ok(JValue::SExpr(vec![]).into_ref())
}

fn jbuiltin_lambda(args: Args, env: JEnvRef) -> JResult {
    let [pvals, code] = get_n_args(args)?;
    let pvals = match &*pvals {
        JValue::SExpr(p) => p,
        _ => return Err(JError::new("TypeError", "expected a list of symbols")),
    };
    let mut params = vec![];
    for val in pvals {
        match &**val {
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
    add_builtin("eq?", jbuiltin_eq, env);
    add_builtin("equal?", jbuiltin_equals, env);
    add_builtin("not", jbuiltin_not, env);
    add_builtin("+", jbuiltin_add, env);
    add_builtin("*", jbuiltin_mul, env);
    add_builtin("error", jbuiltin_error, env);
    add_builtin("raise", jbuiltin_raise, env);
    add_builtin_macro("try", jbuiltin_try, env);
    add_builtin_macro("def", jbuiltin_define, env);
    add_builtin_macro("fn", jbuiltin_lambda, env);
}
