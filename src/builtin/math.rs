use crate::builtin::{get_n_args, get_n_plus_args};
use crate::*;

pub fn jbuiltin_add(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let numbers = args
        .iter_list()?
        .map(Number::from_jval)
        .collect::<Result<Vec<Number>, _>>()?;
    let mut acc = Number::Int(0);
    for n in numbers.iter() {
        acc = acc.add(n)?;
    }
    Ok(acc.to_jval(state))
}

pub fn jbuiltin_sub(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let ([init], rest) = get_n_plus_args(args)?;
    let init = Number::from_jval(init)?;
    let rest = rest
        .into_iter()
        .map(Number::from_jval)
        .collect::<Result<Vec<Number>, JError>>()?;
    if rest.is_empty() {
        Ok(Number::Int(0).sub(&init)?.to_jval(state))
    } else {
        let mut acc = init;
        for n in rest {
            acc = acc.sub(&n)?;
        }
        Ok(acc.to_jval(state))
    }
}

pub fn jbuiltin_mul(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let numbers = args
        .iter_list()?
        .map(Number::from_jval)
        .collect::<Result<Vec<Number>, _>>()?;
    let mut acc = Number::Int(1);
    for n in numbers.iter() {
        acc = acc.mul(n)?;
    }
    Ok(acc.to_jval(state))
}

pub fn jbuiltin_div(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let ([init], rest) = get_n_plus_args(args)?;
    let init = Number::from_jval(init)?;
    let rest = rest
        .into_iter()
        .map(Number::from_jval)
        .collect::<Result<Vec<Number>, JError>>()?;
    if rest.is_empty() {
        Ok(Number::Int(1).div(&init)?.to_jval(state))
    } else {
        let mut acc = init;
        for n in rest {
            acc = acc.div(&n)?;
        }
        Ok(acc.to_jval(state))
    }
}

pub fn jbuiltin_num_eq(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.bool(Number::from_jval(x)?.eq(&Number::from_jval(y)?)?))
}

pub fn jbuiltin_lt(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.bool(Number::from_jval(x)?.lt(&Number::from_jval(y)?)?))
}

pub fn jbuiltin_lte(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.bool(Number::from_jval(x)?.lte(&Number::from_jval(y)?)?))
}

pub fn jbuiltin_gt(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.bool(Number::from_jval(x)?.gt(&Number::from_jval(y)?)?))
}

pub fn jbuiltin_gte(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [x, y] = get_n_args(args)?;
    Ok(state.bool(Number::from_jval(x)?.gte(&Number::from_jval(y)?)?))
}

enum Number {
    Int(JTInt),
    Float(JTFloat),
}

impl Number {
    fn from_jval(val: JValRef) -> Result<Self, JError> {
        match &*val {
            JVal::Int(n) => Ok(Self::Int(*n)),
            JVal::Float(x) => Ok(Self::Float(*x)),
            _ => Err(JError::new(TypeError, "expected a numeric type")),
        }
    }
    fn to_jval(&self, state: &mut JState) -> JValRef {
        match self {
            Self::Int(n) => state.int(*n),
            Self::Float(x) => state.float(*x),
        }
    }
    fn as_float(&self) -> Result<JTFloat, JError> {
        match self {
            Self::Float(x) => Ok(*x),
            Self::Int(n) => {
                let x = *n as JTFloat;
                if x as JTInt != *n {
                    return Err(JError::new(
                        Other("FloatError".to_string()),
                        &format!("cannot convert int {} to float", n),
                    ));
                }
                Ok(x)
            }
        }
    }
    fn add(&self, other: &Self) -> Result<Self, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            let s = n
                .checked_add(*m)
                .ok_or_else(|| JError::new(Other("IntError".to_string()), "overflow"))?;
            Ok(Self::Int(s))
        } else {
            Ok(Self::Float(self.as_float()? + other.as_float()?))
        }
    }
    fn sub(&self, other: &Self) -> Result<Self, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            let s = n
                .checked_sub(*m)
                .ok_or_else(|| JError::new(Other("IntError".to_string()), "overflow"))?;
            Ok(Self::Int(s))
        } else {
            Ok(Self::Float(self.as_float()? - other.as_float()?))
        }
    }
    fn mul(&self, other: &Self) -> Result<Self, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            let s = n
                .checked_mul(*m)
                .ok_or_else(|| JError::new(Other("IntError".to_string()), "overflow"))?;
            Ok(Self::Int(s))
        } else {
            Ok(Self::Float(self.as_float()? * other.as_float()?))
        }
    }
    fn div(&self, other: &Self) -> Result<Self, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            let s = n
                .checked_div(*m)
                .ok_or_else(|| JError::new(Other("IntError".to_string()), "overflow"))?;
            Ok(Self::Int(s))
        } else {
            Ok(Self::Float(self.as_float()? / other.as_float()?))
        }
    }
    fn eq(&self, other: &Self) -> Result<bool, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            Ok(n == m)
        } else {
            Ok(self.as_float()? == other.as_float()?)
        }
    }
    fn lt(&self, other: &Self) -> Result<bool, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            Ok(n < m)
        } else {
            Ok(self.as_float()? < other.as_float()?)
        }
    }
    fn lte(&self, other: &Self) -> Result<bool, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            Ok(n <= m)
        } else {
            Ok(self.as_float()? <= other.as_float()?)
        }
    }
    fn gt(&self, other: &Self) -> Result<bool, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            Ok(n > m)
        } else {
            Ok(self.as_float()? > other.as_float()?)
        }
    }
    fn gte(&self, other: &Self) -> Result<bool, JError> {
        if let (Self::Int(n), Self::Int(m)) = (self, other) {
            Ok(n >= m)
        } else {
            Ok(self.as_float()? >= other.as_float()?)
        }
    }
}
