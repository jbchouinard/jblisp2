use crate::builtin::*;

pub fn jbuiltin_cons(args: Args, _env: JEnvRef) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(JValue::Cell(JCell::cons(Rc::clone(&x), Rc::clone(&y))).into_ref())
}

pub fn jbuiltin_car(args: Args, _env: JEnvRef) -> JResult {
    let [list] = get_n_args(args)?;
    match &*list {
        JValue::Cell(c) => Ok(c.car()?),
        _ => Err(JError::new("TypeError", "expected a cons cell")),
    }
}

pub fn jbuiltin_cdr(args: Args, _env: JEnvRef) -> JResult {
    let [list] = get_n_args(args)?;
    match &*list {
        JValue::Cell(c) => Ok(c.cdr()?),
        _ => Err(JError::new("TypeError", "expected cons cell")),
    }
}
