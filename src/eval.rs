use std::rc::Rc;

use crate::*;

pub fn eval(expr: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    match &*expr {
        JVal::Pair(list) => eval_list(list, env, state),
        JVal::Symbol(sym) => env.try_lookup(sym),
        JVal::Quote(val) => Ok(Rc::clone(val)),
        JVal::Quasiquote(val) => eval_qq(Rc::clone(val), env, state, 1),
        JVal::Unquote(_) => Err(JError::new(EvalError, "misplaced unquote")),
        JVal::UnquoteSplice(_) => Err(JError::new(EvalError, "misplaced unquote-splice")),
        _ => Ok(expr),
    }
}

fn apply_builtin(b: &JBuiltin, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let args = eval_args(args, Rc::clone(&env), state)?;
    (b.f)(args, env, state)
}

fn apply_special_form(b: &JBuiltin, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    (b.f)(args, env, state)
}

pub fn apply_lambda(lambda: &JLambda, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let invoke_env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    let args = eval_args(args, env, state)?;
    lambda.params.bind(args, Rc::clone(&invoke_env))?;
    let mut last_res = state.nil();
    for expr in &lambda.code {
        last_res = eval(Rc::clone(expr), Rc::clone(&invoke_env), state)?;
    }
    Ok(last_res)
}

fn apply_proc_macro(lambda: &JLambda, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let invoke_env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    lambda.params.bind(args, Rc::clone(&invoke_env))?;
    let mut last_res = state.nil();
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

    Ok(state.list(evaluated))
}

fn eval_qq(expr: JValRef, env: JEnvRef, state: &mut JState, qqlvl: u32) -> JResult {
    match (qqlvl, &*expr) {
        (0, _) => eval(expr, env, state),
        (_, JVal::Pair(p)) => eval_qq_list(p, env, state, qqlvl),
        (_, JVal::Quasiquote(qexpr)) => Ok(JVal::Quasiquote(eval_qq(
            Rc::clone(qexpr),
            Rc::clone(&env),
            state,
            qqlvl + 1,
        )?)
        .into_ref()),
        (1, JVal::Unquote(uexpr)) => eval(Rc::clone(uexpr), Rc::clone(&env), state),
        (_, JVal::Unquote(uexpr)) => Ok(JVal::Unquote(eval_qq(
            Rc::clone(uexpr),
            Rc::clone(&env),
            state,
            qqlvl - 1,
        )?)
        .into_ref()),
        _ => Ok(expr),
    }
}

fn eval_qq_list(list: &JPair, env: JEnvRef, state: &mut JState, qqlvl: u32) -> JResult {
    let mut list_out: Vec<JValRef> = vec![];
    for expr in list.iter()? {
        match &*expr {
            JVal::UnquoteSplice(uexpr) => {
                let res = eval_qq(Rc::clone(uexpr), Rc::clone(&env), state, qqlvl - 1)?;
                match &*res {
                    JVal::Pair(p) => {
                        if qqlvl > 1 {
                            list_out.push(JVal::UnquoteSplice(res).into_ref());
                        } else {
                            for v in p.iter()? {
                                list_out.push(v);
                            }
                        }
                    }
                    JVal::Nil => (),
                    _ => return Err(JError::new(TypeError, "unsplice expected a list")),
                }
            }
            _ => list_out.push(eval_qq(expr, Rc::clone(&env), state, qqlvl)?),
        }
    }
    Ok(state.list(list_out))
}

fn eval_list(list: &JPair, env: JEnvRef, state: &mut JState) -> JResult {
    let func = eval(list.car(), Rc::clone(&env), state)?;
    let args = list.cdr();
    let envclone = Rc::clone(&env);
    let res = match &*func {
        JVal::Builtin(b) => apply_builtin(b, args, env, state),
        JVal::SpecialForm(b) => apply_special_form(b, args, env, state),
        JVal::Lambda(l) => apply_lambda(l, args, env, state),
        JVal::Macro(l) => apply_proc_macro(l, args, env, state),
        _ => return Err(JError::new(TypeError, "expected a callable")),
    };
    match res {
        Ok(val) => Ok(val),
        Err(err) => {
            state.traceback_push(TracebackFrame::from_jval(func, envclone));
            Err(err)
        }
    }
}
