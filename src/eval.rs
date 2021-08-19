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

fn eval_list(list: &JPair, env: JEnvRef, state: &mut JState) -> JResult {
    let func = eval(list.car(), Rc::clone(&env), state)?;
    let args = list.cdr();
    func.apply(args, env, state)
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
