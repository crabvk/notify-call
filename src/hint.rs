use dbus::arg::{RefArg, Variant};
use std::{error, fmt, result};

#[derive(Debug)]
pub enum ParseHintError {
    NaN(String),
    UnknownType(String),
    InvalidFormat(String),
}

impl ParseHintError {
    pub fn nan(tnv: Vec<&str>, reason: &dyn error::Error) -> Self {
        let msg = format!(
            "Value \"{}\" of hint \"{}\" could not be parsed as type \"{}\": {}.",
            tnv[2], tnv[1], tnv[0], reason
        );
        Self::NaN(msg)
    }
}

impl fmt::Display for ParseHintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseHintError::NaN(msg) => write!(f, "{}", msg),
            ParseHintError::UnknownType(t) => write!(
                f,
                "Invalid hint type \"{}\". Valid types are int, double, string and byte.",
                t
            ),
            ParseHintError::InvalidFormat(s) => {
                write!(f, "Invalid hint syntax in \"{}\". Use TYPE:NAME:VALUE.", s)
            }
        }
    }
}

impl error::Error for ParseHintError {}

#[derive(Debug)]
pub struct Hint<'a> {
    pub name: &'a str,
    pub value: Variant<Box<dyn RefArg>>,
}

impl<'a> Hint<'a> {
    fn new(name: &'a str, refarg: Box<dyn RefArg>) -> Self {
        Self {
            name,
            value: Variant(refarg),
        }
    }

    pub fn from_str(s: &'a str) -> result::Result<Self, ParseHintError> {
        let tnv: Vec<&'a str> = s.split(':').collect();

        if tnv.len() == 3 && tnv.iter().all(|i| !i.trim().is_empty()) {
            let name = tnv[1];

            match tnv[0] {
                "int" => match tnv[2].parse::<i32>() {
                    Ok(value) => Ok(Hint::new(name, Box::new(value))),
                    Err(e) => Err(ParseHintError::nan(tnv, &e)),
                },
                "byte" => match tnv[2].parse::<u8>() {
                    Ok(value) => Ok(Hint::new(name, Box::new(value))),
                    Err(e) => Err(ParseHintError::nan(tnv, &e)),
                },
                "double" => match tnv[2].parse::<f64>() {
                    Ok(value) => Ok(Hint::new(name, Box::new(value))),
                    Err(e) => Err(ParseHintError::nan(tnv, &e)),
                },
                "string" => {
                    let value = tnv[2].to_string();
                    Ok(Hint::new(name, Box::new(value)))
                }
                unknown => Err(ParseHintError::UnknownType(String::from(unknown))),
            }
        } else {
            Err(ParseHintError::InvalidFormat(String::from(s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hint() {
        let hint = Hint::from_str("int:value:42").unwrap();
        assert_eq!("value", hint.name);
        assert_eq!(42, hint.value.as_i64().unwrap())
    }

    #[test]
    fn parsing_error_nan() {
        let error = Hint::from_str("double:error:wtf").err().unwrap();
        let expect = "Value \"wtf\" of hint \"error\" could not be parsed as type \"double\": invalid float literal.";
        assert_eq!(expect, format!("{}", error));
    }

    #[test]
    fn parsing_error_invalid_format() {
        let error = Hint::from_str("testing...").err().unwrap();
        let expect = "Invalid hint syntax in \"testing...\". Use TYPE:NAME:VALUE.";
        assert_eq!(expect, format!("{}", error));
    }
}
