use crate::*;

pub fn jrepr(expr: JValueRef) -> String {
    match &*expr {
        JValue::Int(n) => format!("{}", n),
        JValue::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
        JValue::Symbol(s) => s.to_string(),
        JValue::String(s) => format!("\"{}\"", s),
        JValue::Error(e) => format!("<error {} \"{}\">", e.etype, e.emsg),
        JValue::Builtin(b) => format!("<function {:?}>", b),
        JValue::BuiltinMacro(b) => format!("<macro {:?}>", b),
        JValue::Lambda(l) => format!("<{}-param lambda>", l.params.len()),
        JValue::Cell(c) => match c {
            JCell::Nil => "()".to_string(),
            JCell::Pair(x, y) => format!("({} . {})", jrepr(Rc::clone(x)), jrepr(Rc::clone(y))),
        },
    }
}
