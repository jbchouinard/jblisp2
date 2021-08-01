use crate::*;

fn repr_sexpr(list: &[JValue]) -> String {
    let mut parts = vec!["(".to_string()];
    for val in list {
        parts.push(jrepr(val));
        parts.push(" ".to_string());
    }
    if !list.is_empty() {
        parts.pop();
    }
    parts.push(")".to_string());
    parts.into_iter().collect()
}

pub fn jrepr(expr: &JValue) -> String {
    match expr {
        JValue::SExpr(list) => repr_sexpr(list),
        JValue::Int(n) => format!("{}", n),
        JValue::Symbol(s) => s.to_string(),
        JValue::Error(e) => format!("{}: {}", e.etype, e.emsg),
        JValue::Builtin(b) => format!("{:?}", b),
    }
}
