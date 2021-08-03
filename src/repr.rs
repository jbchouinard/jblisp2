use crate::*;

pub fn jrepr(expr: &JValue) -> String {
    match &*expr {
        JValue::Int(n) => format!("{}", n),
        JValue::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
        JValue::Symbol(s) => s.to_string(),
        JValue::String(s) => format!("\"{}\"", s),
        JValue::Error(e) => format!("<error {} \"{}\">", e.etype, e.emsg),
        JValue::Builtin(b) => format!("<builtin {:?}>", b),
        JValue::SpecialForm(b) => format!("<special form {:?}>", b),
        JValue::Lambda(l) => format!("<{}-param lambda>", l.params.len()),
        JValue::Macro(l) => format!("<{}-param macro>", l.params.len()),
        JValue::Cell(c) => match c {
            JCell::Nil => "()".to_string(),
            JCell::Pair(x, y) => format!("({} . {})", jrepr(&*x), jrepr(&*y)),
        },
        JValue::Quoted(val) => format!("'{}", jrepr(&*val)),
    }
}
