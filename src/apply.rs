use std::rc::Rc;

use crate::*;

impl JVal {
    pub fn apply(&self, args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
        let res = match self {
            JVal::Builtin(b) => apply_builtin(b, args, Rc::clone(&env), state),
            JVal::SpecialForm(b) => apply_special_form(b, args, Rc::clone(&env), state),
            JVal::Lambda(l) => apply_lambda(l, args, Rc::clone(&env), state),
            JVal::Macro(l) => apply_proc_macro(l, args, Rc::clone(&env), state),
            _ => return Err(JError::new(TypeError, "expected a callable")),
        };
        match res {
            Ok(val) => Ok(val),
            Err(err) => {
                state.traceback_push(TracebackFrame::from_jval(self, env));
                Err(err)
            }
        }
    }
}

fn eval_args(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let evaluated: Vec<JValRef> = args
        .iter_list()?
        .map(|v| eval(v, Rc::clone(&env), state))
        .collect::<Result<Vec<JValRef>, JError>>()?;

    Ok(state.list(evaluated))
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
