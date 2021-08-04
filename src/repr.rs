use crate::*;

pub fn jrepr(expr: &JValue) -> String {
    match &*expr {
        JValue::Int(n) => format!("{}", n),
        JValue::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
        JValue::Symbol(s) => s.to_string(),
        JValue::String(s) => format!("\"{}\"", s),
        JValue::Error(e) => format!("#[error {} \"{}\"]", e.etype, e.emsg),
        JValue::Builtin(b) => format!("#[builtin {:?}]", b),
        JValue::SpecialForm(b) => format!("#[specialform {:?}]", b),
        JValue::Lambda(l) => format!("#[lambda {} {:p}]", l.params.len(), l),
        JValue::Macro(l) => format!("#[macro {} {:p}]", l.params.len(), l),
        JValue::Cell(c) => repr_cell(c),
        JValue::Quoted(val) => format!("'{}", jrepr(&*val)),
    }
}

fn repr_cell(cell: &JCell) -> String {
    match cell.iter() {
        Ok(iterator) => {
            let mut parts = vec!["(".to_string()];
            for val in iterator {
                parts.push(jrepr(&val));
                parts.push(" ".to_string());
            }
            if parts.len() > 1 {
                parts.pop();
            }
            parts.push(")".to_string());
            parts.join("")
        }
        _ => match cell {
            JCell::Nil => "()".to_string(),
            JCell::Pair(x, y) => format!("({} . {})", jrepr(&*x), jrepr(&*y)),
        },
    }
}
