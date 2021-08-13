use std::convert::TryInto;

use crate::builtin::get_n_args;
use crate::*;

pub fn jbuiltin_concat(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut strings: Vec<String> = vec![];
    for arg in args.iter_list()? {
        strings.push(arg.to_str()?.to_owned())
    }
    Ok(state.string(strings.join("")))
}

pub fn jbuiltin_contains(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [string, substring] = get_n_args(args)?;
    let string = string.to_str()?;
    let substring = substring.to_str()?;
    Ok(state.bool(string.contains(substring)))
}

fn normalize_index(mut n: JTInt, length: usize) -> usize {
    let ilen: JTInt = length.try_into().unwrap();
    if n < 0 {
        n += ilen;
        if n < 0 {
            n = 0;
        }
    }
    if n > ilen {
        n = ilen
    }
    n.try_into().unwrap()
}

pub fn jbuiltin_substring(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [string, start, end] = get_n_args(args)?;
    let chars: Vec<char> = string.to_str()?.chars().collect();
    let start = normalize_index(start.to_int()?, chars.len());
    let end = normalize_index(end.to_int()?, chars.len());

    let substring: String = if start <= end {
        chars[start..end].iter().collect()
    } else {
        chars[end..start].iter().rev().collect()
    };

    Ok(state.string(substring))
}

pub fn jbuiltin_len(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [string] = get_n_args(args)?;
    let string = string.to_str()?;
    let len: JTInt = string.chars().count().try_into().unwrap();
    Ok(state.int(len))
}

pub fn jbuiltin_split(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [string, sep] = get_n_args(args)?;
    let string = string.to_str()?;
    let sep = sep.to_str()?;
    let mut list = vec![];
    for part in string.split(sep) {
        list.push(state.string(part.to_string()))
    }
    Ok(state.list(list))
}

pub fn jbuiltin_replace(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [string, src, target] = get_n_args(args)?;
    let string = string.to_str()?;
    let src = src.to_str()?;
    let target = target.to_str()?;
    Ok(state.string(string.replace(src, target)))
}

pub fn jbuiltin_parse_int(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [s] = get_n_args(args)?;
    let s = s.to_str()?;
    let int = s
        .parse::<JTInt>()
        .map_err(|e| JError::new(Other("IntError".to_string()), &format!("{}", e)))?;
    Ok(state.int(int))
}

pub fn jbuiltin_parse_float(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [s] = get_n_args(args)?;
    let s = s.to_str()?;
    let float = s
        .parse::<JTFloat>()
        .map_err(|e| JError::new(Other("FloatError".to_string()), &format!("{}", e)))?;
    Ok(state.float(float))
}
