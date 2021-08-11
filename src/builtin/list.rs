use crate::builtin::*;

pub fn jbuiltin_cons(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.pair(Rc::clone(&x), Rc::clone(&y)))
}

pub fn jbuiltin_car(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [pair] = get_n_args(args)?;
    let pair = pair.to_pair()?;
    Ok(pair.car())
}

pub fn jbuiltin_cdr(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [pair] = get_n_args(args)?;
    let pair = pair.to_pair()?;
    Ok(pair.cdr())
}

pub fn jbuiltin_list(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    Ok(args)
}

pub fn jbuiltin_is_list(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [pair] = get_n_args(args)?;
    let pair = pair.to_pair()?;
    Ok(state.bool(pair.is_list()))
}

pub fn jspecial_quote(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [x] = get_n_args(args)?;
    Ok(Rc::clone(&x))
}
