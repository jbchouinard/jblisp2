use crate::builtin::*;

pub fn jbuiltin_cons(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.jpair(Rc::clone(&x), Rc::clone(&y)))
}

pub fn jbuiltin_car(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [list] = get_n_args(args)?;
    match &*list {
        JVal::Pair(c) => Ok(c.car()),
        _ => Err(JError::new(TypeError, "expected a pair")),
    }
}

pub fn jbuiltin_cdr(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [list] = get_n_args(args)?;
    match &*list {
        JVal::Pair(c) => Ok(c.cdr()),
        _ => Err(JError::new(TypeError, "expected a pair")),
    }
}

pub fn jbuiltin_list(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    Ok(args)
}

pub fn jbuiltin_is_list(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    Ok(state.jbool(match &*val {
        JVal::Pair(c) => c.is_list(),
        _ => false,
    }))
}

pub fn jspecial_quote(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [x] = get_n_args(args)?;
    Ok(Rc::clone(&x))
}
