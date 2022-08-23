use super::data::INCAR_TAGS;
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub trait ValidIncarValueType {}

impl ValidIncarValueType for String {}
impl ValidIncarValueType for bool {}
impl ValidIncarValueType for i32 {}
impl ValidIncarValueType for f64 {}
impl ValidIncarValueType for Vec<f64> {}
impl ValidIncarValueType for Vec<i64> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncarTagType {
    Bool,
    Integer,
    IntegerArray,
    Real,
    RealArray,
    String,
}

#[derive(Debug)]
pub enum IncarTag {
    Boolean(String, bool),
    Integer(String, i64),
    IntegerArray(String, Vec<i64>),
    Real(String, f64),
    RealArray(String, Vec<f64>),
    String(String, String),
}

impl IncarTag {
    pub fn name(&self) -> &str {
        match self {
            IncarTag::Boolean(name, _) => name,
            IncarTag::Integer(name, _) => name,
            IncarTag::IntegerArray(name, _) => name,
            IncarTag::Real(name, _) => name,
            IncarTag::RealArray(name, _) => name,
            IncarTag::String(name, _) => name,
        }
    }
    pub fn is_name(&self, s: &str) -> bool {
        s == self.name()
    }
}

impl FromStr for IncarTag {
    type Err = String;
    /// s: ex) "ISIF    =  3"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, val) = {
            let mut iter = s
                .split('#')
                .next()
                .ok_or(String::from("Invalide tag format"))?
                .splitn(2, '=');
            (
                iter.next().ok_or(String::from("Invalide tag format"))?.trim(),
                iter.next().ok_or(String::from("Invalide tag format"))?.trim(),
            )
        };
        let tagtype = &INCAR_TAGS
            .get(name)
            .ok_or(format!("{} is not a valid INCAR tag", name))?;

        match tagtype {
            IncarTagType::String => Ok(IncarTag::String(name.into(), val.into())),
            IncarTagType::Integer => Ok(IncarTag::Integer(name.into(), parse_single_val(val)?)),
            IncarTagType::Real => Ok(IncarTag::Real(name.into(), parse_single_val(val)?)),
            IncarTagType::Bool => Ok(IncarTag::Boolean(name.into(), parse_bool(val)?)),
            IncarTagType::IntegerArray => Ok(IncarTag::IntegerArray(name.into(), parse_array(val)?)),
            IncarTagType::RealArray => Ok(IncarTag::RealArray(name.into(), parse_array(val)?)),
        }
    }
}

impl ToString for IncarTag {
    fn to_string(&self) -> String {
        match &self {
            IncarTag::Boolean(name, val) => {
                let bool_str = match val {
                    true => ".TRUE.",
                    false => ".FALSE.",
                };
                format!("{} = {}", name, bool_str)
            }
            IncarTag::Integer(name, val) => format!("{} = {}", name, val),
            IncarTag::Real(name, val) => format!("{} = {}", name, val),
            IncarTag::String(name, val) => format!("{} = {}", name, val),
            IncarTag::IntegerArray(name, val) => {
                let arr_repr = val
                    .into_iter()
                    .map(|x| format!("{} = {}", name, x))
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("{} = {}", name, arr_repr)
            }
            IncarTag::RealArray(name, val) => {
                let arr_repr = val
                    .into_iter()
                    .map(|x| format!("{:.6}", x))
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("{} = {}", name, arr_repr)
            }
        }
    }
}

fn parse_single_val<T>(val_str: &str) -> Result<T, String>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + Display,
{
    let val = val_str.parse::<T>().map_err(|e| e.to_string())?;
    Ok(val)
}

fn parse_bool(val_str: &str) -> Result<bool, String> {
    match val_str {
        "T" | "true" | "True" | ".TRUE." => Ok(true),
        "F" | "false" | "False" | ".FALSE." => Ok(false),
        _ => Err(format!("{} is not a valid boolean value", val_str)),
    }
}

fn parse_array<T>(val_str: &str) -> Result<Vec<T>, String>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + Display,
{
    let mut iter = val_str.trim().split_whitespace();
    let mut vec = Vec::new();
    while let Some(val) = iter.next() {
        vec.push(val.trim().parse::<T>().map_err(|e| e.to_string())?);
    }
    Ok(vec)
}
