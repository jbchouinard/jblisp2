use crate::*;

fn repr_sexpr(list: &[JValueRef]) -> String {
    let mut parts = vec!["(".to_string()];
    for val in list {
        parts.push(jrepr(Rc::clone(val)));
        parts.push(" ".to_string());
    }
    if !list.is_empty() {
        parts.pop();
    }
    parts.push(")".to_string());
    parts.into_iter().collect()
}

pub fn jrepr(expr: JValueRef) -> String {
    match &*expr {
        JValue::SExpr(list) => repr_sexpr(list),
        JValue::Int(n) => format!("{}", n),
        JValue::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
        JValue::Symbol(s) => s.to_string(),
        JValue::String(s) => format!("\"{}\"", s),
        JValue::Error(e) => format!("<error {} \"{}\">", e.etype, e.emsg),
        JValue::Builtin(b) => format!("<function {:?}>", b),
        JValue::BuiltinMacro(b) => format!("<macro {:?}>", b),
        JValue::Lambda(l) => format!("<{}-param lambda>", l.params.len()),
        JValue::Cell(_) => "todo".to_string(),
    }
}
