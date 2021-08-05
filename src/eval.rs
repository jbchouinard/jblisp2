use std::rc::Rc;

use crate::*;

pub fn jeval(expr: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    match &*expr {
        JVal::Quoted(val) => Ok(Rc::clone(val)),
        JVal::Pair(c) => eval_sexpr(c, env, state),
        JVal::Symbol(sym) => match env.lookup(sym) {
            Some(val) => Ok(val),
            None => Err(JError::new("Undefined", sym)),
        },
        JVal::Lambda(_) => Ok(expr),
        _ => Ok(expr),
    }
}

fn eval_sexpr(list: &JPair, env: JEnvRef, state: &mut JState) -> JResult {
    let func = jeval(list.car(), Rc::clone(&env), state)?;
    let args = list.cdr();
    match &*func {
        JVal::Builtin(b) => apply_builtin(b, args, env, state),
        JVal::SpecialForm(b) => apply_special_form(b, args, env, state),
        JVal::Lambda(l) => apply_lambda(l, args, env, state),
        JVal::Macro(l) => apply_macro(l, args, env, state),
        _ => Err(JError::new("TypeError", "expected a callable")),
    }
}

fn apply_builtin(b: &JBuiltin, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let (_, args) = eval_args(args, Rc::clone(&env), state)?;
    (b.f)(args, env, state)
}

fn apply_special_form(b: &JBuiltin, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    (b.f)(args, env, state)
}

// TODO: currying?
fn apply_lambda(lambda: &JLambda, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let (n, args) = eval_args(args, env, state)?;
    if n != lambda.params.len() {
        return Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", lambda.params.len()),
        ));
    }
    // Create environment to bind the arguments
    let env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    for (name, val) in lambda.params.iter().zip(args.iter_list()?) {
        env.define(name, val);
    }
    jeval(Rc::clone(&lambda.code), env, state)
}

fn apply_macro(lambda: &JLambda, args: JValRef, fenv: JEnvRef, state: &mut JState) -> JResult {
    if args.iter_list()?.count() != lambda.params.len() {
        return Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", lambda.params.len()),
        ));
    }
    // Create environment to bind the arguments
    let env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    for (name, val) in lambda.params.iter().zip(args.iter_list()?) {
        env.define(name, val);
    }
    let code = jeval(Rc::clone(&lambda.code), env, state)?;
    jeval(code, fenv, state)
}

fn eval_args(args: JValRef, env: JEnvRef, state: &mut JState) -> Result<(usize, JValRef), JError> {
    let evaluated: Vec<JValRef> = args
        .iter_list()?
        .map(|v| jeval(v, Rc::clone(&env), state))
        .collect::<Result<Vec<JValRef>, JError>>()?;

    let n = evaluated.len();

    Ok((n, JVal::list(evaluated)))
}
