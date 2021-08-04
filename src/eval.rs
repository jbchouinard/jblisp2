use std::rc::Rc;

use crate::*;

pub fn jeval(expr: JValueRef, env: JEnvRef) -> JResult {
    match &*expr {
        JValue::Quoted(val) => Ok(Rc::clone(val)),
        JValue::Cell(c) => eval_sexpr(c, env),
        JValue::Symbol(sym) => match env.lookup(sym) {
            Some(val) => Ok(val),
            None => Err(JError::new("Undefined", sym)),
        },
        JValue::Lambda(_) => Ok(expr),
        _ => Ok(expr),
    }
}

fn eval_sexpr(list: &JCell, env: JEnvRef) -> JResult {
    if list.is_nil() {
        Ok(JValue::Cell(JCell::Nil).into_ref())
    } else {
        let func = list.car()?;
        let func = jeval(func, Rc::clone(&env))?;
        let args = list.cdr()?;
        let args = match &*args {
            JValue::Cell(c) => c,
            _ => return Err(JError::new("EvalError", "not a list")),
        };

        match &*func {
            JValue::Builtin(b) => apply_builtin(b, args, env),
            JValue::SpecialForm(b) => apply_special_form(b, args, env),
            JValue::Lambda(l) => apply_lambda(l, args, env),
            JValue::Macro(l) => apply_macro(l, args, env),
            _ => Err(JError::new("TypeError", "expected a callable")),
        }
    }
}

fn apply_builtin(b: &JBuiltin, args: &JCell, env: JEnvRef) -> JResult {
    let (_, args) = eval_args(args, Rc::clone(&env))?;
    (b.f)(&args, env)
}

fn apply_special_form(b: &JBuiltin, args: &JCell, env: JEnvRef) -> JResult {
    (b.f)(args, env)
}

// TODO: currying?
fn apply_lambda(lambda: &JLambda, args: &JCell, env: JEnvRef) -> JResult {
    let (n, args) = eval_args(args, env)?;
    if n != lambda.params.len() {
        return Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", lambda.params.len()),
        ));
    }
    // Create environment to bind the arguments
    let env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    for (name, val) in lambda.params.iter().zip(args.iter()?) {
        env.define(name, val);
    }
    jeval(Rc::clone(&lambda.code), env)
}

fn apply_macro(lambda: &JLambda, args: &JCell, fenv: JEnvRef) -> JResult {
    if args.iter()?.count() != lambda.params.len() {
        return Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", lambda.params.len()),
        ));
    }
    // Create environment to bind the arguments
    let env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    for (name, val) in lambda.params.iter().zip(args.iter()?) {
        env.define(name, val);
    }
    let code = jeval(Rc::clone(&lambda.code), env)?;
    jeval(code, fenv)
}

fn eval_args(args: &JCell, env: JEnvRef) -> Result<(usize, JCell), JError> {
    let mut evaluated = vec![];
    for arg in args.iter()? {
        evaluated.push(jeval(arg, Rc::clone(&env))?);
    }
    Ok((evaluated.len(), vec_to_list(evaluated)))
}
