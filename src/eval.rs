use std::rc::Rc;

use crate::*;

fn apply_builtin(b: &JBuiltin, args: Vec<JValueRef>, env: JEnvRef) -> JResult {
    let mut evaluated = vec![];
    for arg in args {
        evaluated.push(jeval(arg, Rc::clone(&env))?);
    }
    (b.f)(evaluated, env)
}

fn apply_builtin_macro(b: &JBuiltin, args: Vec<JValueRef>, env: JEnvRef) -> JResult {
    (b.f)(args, env)
}

// TODO: currying?
fn apply_lambda(lambda: &JLambda, args: Vec<JValueRef>, _env: JEnvRef) -> JResult {
    if args.len() != lambda.params.len() {
        return Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", lambda.params.len()),
        ));
    }
    // Create environment to bind the arguments
    let env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    for (name, val) in lambda.params.iter().zip(args.into_iter()) {
        env.define(name, val);
    }
    jeval(Rc::clone(&lambda.code), env)
}

fn eval_sexpr(list: &JCell, env: JEnvRef) -> JResult {
    let mut list: Vec<JValueRef> = list.iter()?.collect();
    if list.is_empty() {
        Ok(JValue::Cell(JCell::Nil).into_ref())
    } else {
        let args = list.split_off(1);
        let func = list.pop().unwrap();
        let func = jeval(func, Rc::clone(&env))?;

        match &*func {
            JValue::Builtin(b) => apply_builtin(b, args, env),
            JValue::BuiltinMacro(b) => apply_builtin_macro(b, args, env),
            JValue::Lambda(l) => apply_lambda(l, args, env),
            _ => Err(JError::new("TypeError", "expected a callable")),
        }
    }
}

pub fn jeval(expr: JValueRef, env: JEnvRef) -> JResult {
    match &*expr {
        JValue::Cell(c) => eval_sexpr(c, env),
        JValue::Symbol(sym) => match env.lookup(sym) {
            Some(val) => Ok(val),
            None => Err(JError::new("Undefined", sym)),
        },
        JValue::Lambda(_) => Ok(expr),
        _ => Ok(expr),
    }
}
