use std::convert::TryInto;
use std::rc::Rc;

use crate::*;

fn jbuiltin_add(args: Vec<JValue>, _env: &mut JEnv) -> JResult {
    let mut sum: JTInt = 0;
    for arg in args {
        match arg.into_int() {
            Ok(n) => {
                sum += n;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(JValue::Int(sum))
}

fn jbuiltin_define(args: Vec<JValue>, env: &mut JEnv) -> JResult {
    let [jsym, jval] = get_n_args(args)?;
    let sym = match jsym {
        JValue::Symbol(s) => s,
        _ => return Err(JError::new("TypeError", "expected a symbol")),
    };
    let jval = jeval(jval, env)?;
    env.set(&sym, jval);
    Ok(JValue::SExpr(vec![]))
}

fn jbuiltin_lambda(args: Vec<JValue>, env: &mut JEnv) -> JResult {
    let [pvals, code] = get_n_args(args)?;
    let pvals = match pvals {
        JValue::SExpr(p) => p,
        _ => return Err(JError::new("TypeError", "expected a list of symbols")),
    };
    let mut params = vec![];
    for val in pvals {
        match val {
            JValue::Symbol(s) => params.push(s),
            _ => return Err(JError::new("TypeError", "expected a list of symbols")),
        }
    }
    let mut closure = JEnv::new();
    closure.set_parent(Some(Box::new(env.clone())));
    Ok(JValue::Lambda(Box::new(JLambda {
        closure,
        code,
        params,
    })))
}

fn add_builtin<T>(name: &str, f: T, env: &mut JEnv)
where
    T: 'static + Fn(Vec<JValue>, &mut JEnv) -> JResult,
{
    let val = JValue::Builtin(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.set(name, val);
}

fn add_builtin_macro<T>(name: &str, f: T, env: &mut JEnv)
where
    T: 'static + Fn(Vec<JValue>, &mut JEnv) -> JResult,
{
    let val = JValue::BuiltinMacro(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.set(name, val);
}

pub fn add_builtins(env: &mut JEnv) {
    add_builtin("+", jbuiltin_add, env);
    add_builtin_macro("define", jbuiltin_define, env);
    add_builtin_macro("fn", jbuiltin_lambda, env);
}

fn get_n_args<const N: usize>(args: Vec<JValue>) -> Result<[JValue; N], JError> {
    if args.len() == N {
        Ok(args.try_into().unwrap())
    } else {
        Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", N),
        ))
    }
}
