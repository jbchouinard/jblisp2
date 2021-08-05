use crate::*;

pub fn jrepr(expr: &JVal) -> String {
    match &*expr {
        JVal::Nil => "()".to_string(),
        JVal::Int(n) => format!("{}", n),
        JVal::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
        JVal::Symbol(s) => s.to_string(),
        JVal::String(s) => format!("\"{}\"", s),
        JVal::Error(e) => format!("#[error {} \"{}\"]", e.etype, e.emsg),
        JVal::Builtin(b) => format!("#[builtin {:?}]", b),
        JVal::SpecialForm(b) => format!("#[specialform {:?}]", b),
        JVal::Lambda(l) => format!("#[lambda {} {:p}]", l.params.len(), l),
        JVal::Macro(l) => format!("#[macro {} {:p}]", l.params.len(), l),
        JVal::Pair(c) => repr_cell(c),
        JVal::Quoted(val) => format!("'{}", jrepr(&*val)),
    }
}

fn repr_cell(cell: &JPair) -> String {
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
        // Not a list
        Err(_) => format!("({} . {})", jrepr(&cell.car()), jrepr(&cell.cdr())),
    }
}
