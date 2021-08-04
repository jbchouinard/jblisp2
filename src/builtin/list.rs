use crate::builtin::*;

pub fn jbuiltin_cons(args: &JCell, _env: JEnvRef) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(JValue::Cell(JCell::cons(Rc::clone(&x), Rc::clone(&y))).into_ref())
}

pub fn jbuiltin_car(args: &JCell, _env: JEnvRef) -> JResult {
    let [list] = get_n_args(args)?;
    match &*list {
        JValue::Cell(c) => Ok(c.car()?),
        _ => Err(JError::new("TypeError", "expected a cons cell")),
    }
}

pub fn jbuiltin_cdr(args: &JCell, _env: JEnvRef) -> JResult {
    let [list] = get_n_args(args)?;
    match &*list {
        JValue::Cell(c) => Ok(c.cdr()?),
        _ => Err(JError::new("TypeError", "expected cons cell")),
    }
}

pub fn jbuiltin_list(args: &JCell, _env: JEnvRef) -> JResult {
    Ok(JValue::Cell(args.clone()).into_ref())
}

pub fn jbuiltin_is_list(args: &JCell, _env: JEnvRef) -> JResult {
    let [val] = get_n_args(args)?;
    Ok(JValue::Bool(match &*val {
        JValue::Cell(c) => c.is_list(),
        _ => false,
    })
    .into_ref())
}

pub fn jspecial_quote(args: &JCell, _env: JEnvRef) -> JResult {
    let [x] = get_n_args(args)?;
    Ok(Rc::clone(&x))
}
