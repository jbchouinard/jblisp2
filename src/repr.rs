use crate::*;

pub fn repr(expr: &JVal) -> String {
    match &*expr {
        JVal::Nil => "()".to_string(),
        JVal::Int(n) => format!("{}", n),
        JVal::Float(x) => format!("{}", x),
        JVal::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
        JVal::Symbol(s) => s.to_string(),
        JVal::String(s) => format!("\"{}\"", s),
        JVal::Error(e) => format!("#[error {}]", e),
        JVal::Builtin(b) => format!("#[function {}]", b),
        JVal::SpecialForm(b) => format!("#[specialform {}]", b),
        JVal::Lambda(l) => format!("#[lambda {}]", l),
        JVal::Macro(l) => format!("#[macro {}]", l),
        JVal::Pair(c) => repr_pair(c),
        JVal::Vector(v) => repr_vec(v),
        JVal::Quote(val) => format!("'{}", repr(&*val)),
        JVal::Quasiquote(val) => format!("`{}", repr(&*val)),
        JVal::Unquote(val) => format!(",{}", repr(&*val)),
        JVal::UnquoteSplice(val) => format!(",@{}", repr(&*val)),
        JVal::Env(env) => format!("{}", env),
        JVal::Token(t) => format!("#[token {}]", t),
        JVal::TokenMatcher(tm) => format!("#[tokenmatcher {}]", tm),
    }
}

fn repr_vec(v: &JVector) -> String {
    let vecref = v.borrow();
    format!(
        "#({})",
        vecref
            .iter()
            .map(|v| repr(v))
            .collect::<Vec<String>>()
            .join(" ")
    )
}

fn repr_pair(cell: &JPair) -> String {
    match cell.iter() {
        Ok(iterator) => {
            format!(
                "({})",
                iterator
                    .map(|v| repr(&v))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        // Not a list
        Err(_) => format!("({} . {})", repr(&cell.car()), repr(&cell.cdr())),
    }
}
