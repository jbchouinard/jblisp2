use std::rc::Rc;

use crate::builtin::get_n_args;
use crate::*;

pub fn jbuiltin_env(args: JValRef, env: JEnvRef, _state: &mut JState) -> JResult {
    let [] = get_n_args(args)?;
    // Take parent cause we don't really wants to return the invocation env of this
    // function.
    Ok(JVal::Env(env).into_ref())
}

pub fn jbuiltin_env_parent(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [env] = get_n_args(args)?;
    let env = env.to_env()?;
    Ok(match &env.parent {
        Some(penv) => JVal::Env(Rc::clone(penv)).into_ref(),
        None => state.jnil(),
    })
}

pub fn jbuiltin_env_lookup(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [env, sym] = get_n_args(args)?;
    let env = env.to_env()?;
    let sym = sym.to_symbol()?;
    env.try_lookup(sym)
}

pub fn jbuiltin_env_def(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [env, sym, val] = get_n_args(args)?;
    let env = env.to_env()?;
    let sym = sym.to_symbol()?;
    env.define(sym, val);
    Ok(state.jnil())
}

pub fn jbuiltin_env_set(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [env, sym, val] = get_n_args(args)?;
    let env = env.to_env()?;
    let sym = sym.to_symbol()?;
    env.set(sym, val, state)?;
    Ok(state.jnil())
}
