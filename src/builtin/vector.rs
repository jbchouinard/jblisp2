use std::cell::RefCell;
use std::rc::Rc;

use crate::builtin::{add_builtin, get_n_args};
use crate::*;

fn bounded(length: usize, n: JTInt) -> Result<usize, JError> {
    if n < 0 {
        return Err(JError::new(
            Other("OutOfBounds".to_string()),
            "negative index",
        ));
    }
    let n = n as usize;
    if n >= length {
        Err(JError::new(
            Other("OutOfBounds".to_string()),
            "index too large",
        ))
    } else {
        Ok(n)
    }
}

fn jbuiltin_new(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let v: Vec<JValRef> = args.iter_list()?.collect();
    Ok(JVal::Vector(RefCell::new(v)).into_ref())
}

fn jbuiltin_sub(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [vec, from, to] = get_n_args(args)?;
    let vec = vec.to_vector()?;
    let vecref = vec.borrow();
    let vl = vecref.len();
    let from = bounded(vl + 1, from.to_int()?)?;
    let to = bounded(vl + 1, to.to_int()?)?;
    let res: Vec<JValRef> = vecref[from..to].iter().map(Rc::clone).collect();
    Ok(JVal::Vector(RefCell::new(res)).into_ref())
}

fn jbuiltin_push(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [vec, val] = get_n_args(args)?;
    let vec = vec.to_vector()?;
    let mut vecmut = vec.borrow_mut();
    vecmut.push(val);
    Ok(state.nil())
}

fn jbuiltin_pop(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [vec] = get_n_args(args)?;
    let vec = vec.to_vector()?;
    let mut vecmut = vec.borrow_mut();
    let val = vecmut
        .pop()
        .ok_or_else(|| JError::new(ApplyError, "empty vector"))?;
    Ok(val)
}

fn jbuiltin_set(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [vec, n, val] = get_n_args(args)?;
    let mut vec = vec.to_vector()?.borrow_mut();
    let n = bounded(vec.len(), n.to_int()?)?;
    vec[n] = val;
    Ok(state.nil())
}

fn jbuiltin_get(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [vec, n] = get_n_args(args)?;
    let vec = vec.to_vector()?.borrow_mut();
    let n = bounded(vec.len(), n.to_int()?)?;
    let val = Rc::clone(&vec[n]);
    Ok(val)
}

fn jbuiltin_len(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [vec] = get_n_args(args)?;
    let vec = vec.to_vector()?.borrow();
    let n = vec.len() as JTInt;
    Ok(state.int(n))
}

fn jbuiltin_map(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [vec, f] = get_n_args(args)?;
    let vec = vec.to_vector()?.borrow();
    let mut res = vec![];
    for val in vec.iter() {
        let args = state.list(vec![Rc::clone(val)]);
        res.push(f.apply(args, Rc::clone(&env), state)?);
    }
    Ok(JVal::Vector(RefCell::new(res)).into_ref())
}

pub fn vector_mod(env: JEnvRef, state: &mut JState) -> JValRef {
    let vecmod = JEnv::new(Some(Rc::clone(&env))).into_ref();
    add_builtin("new", jbuiltin_new, &vecmod, state);
    add_builtin("sub", jbuiltin_sub, &vecmod, state);
    add_builtin("get", jbuiltin_get, &vecmod, state);
    add_builtin("set!", jbuiltin_set, &vecmod, state);
    add_builtin("push!", jbuiltin_push, &vecmod, state);
    add_builtin("pop!", jbuiltin_pop, &vecmod, state);
    add_builtin("len", jbuiltin_len, &vecmod, state);
    add_builtin("map", jbuiltin_map, &vecmod, state);
    JVal::Env(vecmod).into_ref()
}
