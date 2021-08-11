use std::rc::Rc;

use crate::state::TbFrame;
use crate::*;

pub fn eval(expr: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    match &*expr {
        JVal::Pair(list) => apply(list, env, state),
        JVal::Symbol(sym) => env.try_lookup(sym),
        JVal::Quote(val) => Ok(Rc::clone(val)),
        _ => Ok(expr),
    }
}

fn apply(list: &JPair, env: JEnvRef, state: &mut JState) -> JResult {
    let func = eval(list.car(), Rc::clone(&env), state)?;
    let args = list.cdr();
    let envclone = Rc::clone(&env);
    let res = match &*func {
        JVal::Builtin(b) => apply_builtin(b, args, env, state),
        JVal::SpecialForm(b) => apply_special_form(b, args, env, state),
        JVal::Lambda(l) => apply_lambda(l, args, env, state),
        JVal::Macro(l) => apply_macro(l, args, env, state),
        _ => return Err(JError::new(TypeError, "expected a callable")),
    };
    match res {
        Ok(val) => Ok(val),
        Err(err) => {
            state.traceback_push(TbFrame::from_any(func, envclone));
            Err(err)
        }
    }
}

fn apply_builtin(b: &JBuiltin, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let args = eval_args(args, Rc::clone(&env), state)?;
    (b.f)(args, env, state)
}

fn apply_special_form(b: &JBuiltin, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    (b.f)(args, env, state)
}

// TODO: currying?
fn apply_lambda(lambda: &JLambda, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let invoke_env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    let args = eval_args(args, env, state)?;
    lambda.params.bind(args, Rc::clone(&invoke_env))?;
    let mut last_res = state.jnil();
    for expr in &lambda.code {
        last_res = eval(Rc::clone(expr), Rc::clone(&invoke_env), state)?;
    }
    Ok(last_res)
}

fn apply_macro(lambda: &JLambda, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let invoke_env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    lambda.params.bind(args, Rc::clone(&invoke_env))?;
    let mut last_res = state.jnil();
    for expr in &lambda.code {
        last_res = eval(Rc::clone(expr), Rc::clone(&invoke_env), state)?;
    }
    eval(last_res, env, state)
}

fn eval_args(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let evaluated: Vec<JValRef> = args
        .iter_list()?
        .map(|v| eval(v, Rc::clone(&env), state))
        .collect::<Result<Vec<JValRef>, JError>>()?;

    Ok(state.jlist(evaluated))
}
