#![allow(dead_code)]
use std::{str::FromStr, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct JsonNumber {
    pub(crate) integer: i128,
    pub(crate) frac: f64,
    pub(crate) exp: i128
}

pub trait FromJson: Sized {
    type Err;
    fn from_json(json: &JsonValue) -> Result<Self, Self::Err>; 
}

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Number(f64),
    String(String),
    Objects(Vec<JsonObject>),
    Boolean(bool),
    Array(Vec<JsonValue>),
    Null
}

impl JsonValue {
    pub fn new<T>(value : T) -> JsonValue where T: Into<Self>{
        value.into()
    }

    pub fn is_number(&self) -> bool {
        if let JsonValue::Number(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let JsonValue::String(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_objects(&self) -> bool {
        if let JsonValue::Objects(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_bool(&self) -> bool {
        if let JsonValue::Boolean(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_null(&self) -> bool {
        println!("{:?}", self);
        match self {
            JsonValue::Null => true,
            _ => false
        }
    }

    pub fn is_array(&self) -> bool {
        if let JsonValue::Array(_) = self {
            true
        } else {
            false
        }
    }

    pub fn deserialize<F: FromJson>(&self) -> Result<F, F::Err> {
        FromJson::from_json(self)
    }
}

impl From<f64> for JsonValue {
    fn from(v: f64) -> Self {
        Self::Number(v)
    }
}

impl TryInto<f64> for JsonValue {
    type Error = ();

    fn try_into(self) -> Result<f64, Self::Error> {
        if let JsonValue::Number(n) = self {
            Ok(n)
        } else {
            Err(())
        }
    }
}

impl From<&str> for JsonValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<String> for JsonValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<JsonObject> for JsonValue {
    fn from(o: JsonObject) -> Self {
        Self::Objects(Vec::from([o]))
    }
}

impl From<Vec<JsonObject>> for JsonValue {
    fn from(o: Vec<JsonObject>) -> Self {
        Self::Objects(o)
    }
}

impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<Vec<Self>> for JsonValue {
    fn from(v: Vec<Self>) -> Self {
        Self::Array(v)
    }
}

impl<T> From<Option<T>> for JsonValue where T: Into<JsonValue> {
    fn from(op: Option<T>) -> Self {
        match op {
            Some(o) => o.into(),
            None => JsonValue::Null
        }
    }
}

impl<T> From<(String, T)> for JsonValue where T: Into<JsonValue> {
    fn from(value: (String, T)) -> Self {
        Self::Objects(Vec::from([value.into()]))
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Objects(o) => write!(f, "{{{}}}", o.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Array(a) => write!(f, "[{}]", a.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")),
            Self::Null => write!(f, "null"),
        }
    }
}

impl FromStr for JsonValue {
    type Err = crate::parser::JsonParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = crate::lexer::JsonLexer::new(s).tokenize();
        match tokens {
            Ok(tokens) => {
                return crate::parser::JsonParser::new(tokens).get_value();
            },
            Err(e) => {
                return Err(crate::parser::JsonParseError::new(format!("lexer error: \"{}\"",e.message)));
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct JsonObject {
    pub name: String,
    pub value: JsonValue
} 

impl JsonObject {
    pub fn new<T>(name: &str, value: T) -> JsonObject where T: Into<JsonValue> {
        JsonObject {name: name.to_string() , value: value.into()}
    }
}

impl<T> From<(&str, T)> for JsonObject where T: Into<JsonValue> {
    fn from(value: (&str, T)) -> Self {
        Self { name: value.0.to_string(), value : value.1.into() }
    }
}

impl<T> From<(String, T)> for JsonObject where T: Into<JsonValue> {
    fn from(value: (String, T)) -> Self {
        Self { name: value.0, value : value.1.into() }
    }
}

impl Display for JsonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\":{}", self.name, self.value.to_string())
    }
}