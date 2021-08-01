use crate::*;

fn eval_sexpr(list: Vec<JValue>, env: &mut JEnv) -> JValue {
    if list.is_empty() {
        JValue::SExpr(vec![])
    } else {
        let mut list: Vec<JValue> = list.into_iter().map(|v| jeval(v, env)).collect();
        let args = list.split_off(1);
        let func = list.pop().unwrap();
        match func {
            JValue::Builtin(b) => (b.f)(args),
            _ => JValue::Error(JError::new("TypeError", "expected a callable")),
        }
    }
}

pub fn jeval(expr: JValue, env: &mut JEnv) -> JValue {
    match expr {
        JValue::SExpr(list) => eval_sexpr(list, env),
        JValue::Int(_) => expr,
        JValue::Symbol(sym) => match env.get(&sym) {
            Some(val) => val,
            None => JValue::Error(JError::new("Undefined", &sym)),
        },
        JValue::Error(_) => expr,
        JValue::Builtin(_) => expr,
    }
}
