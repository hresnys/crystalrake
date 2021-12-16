#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub enum JsonCharToken {
    UnEscaped(String),
    Escape(String),
    Unicode(u16)
}

#[derive(Debug, PartialEq, Eq)]
pub struct JsonNumberToken {
    is_minus: bool,
    integer: String,
    frac: String,
    exp: String
}
use crate::{json::JsonNumber, parser::JsonParseError};
impl JsonNumberToken {
    
    pub fn to_jsonvalue(&self) -> Result<JsonNumber, JsonParseError> {
        let integer : i128 = match self.integer.parse() {
            Ok(i) => i,
            Err(e) => { return Err(JsonParseError::new(e.to_string())); }
        };

        let frac = match self.frac.parse::<f64>() {
            Ok(i) => i,
            Err(e) => { return Err(JsonParseError::new(e.to_string())); }
        };

        let exp = match self.exp.parse::<i128>() {
            Ok(i) => i,
            Err(e) => { return Err(JsonParseError::new(e.to_string())); }
        };
        Ok(JsonNumber { integer, frac, exp })
    }
}

impl Disp for JsonNumberToken {
    fn to_string(&self) -> String {
        let mut s = String::new();
        if self.is_minus { s.push('-'); }
        s.push_str(&self.integer);
        if !self.frac.is_empty() { s.push('.'); }
        s.push_str(&self.frac);
        if !self.exp.is_empty() {s.push('E'); }
        s.push_str(&self.exp);
        s
    }
}

impl JsonNumberToken {
    pub fn new(is_minus: bool) -> JsonNumberToken {
        JsonNumberToken { 
            is_minus, 
            integer: String::new(), 
            frac: String::new(), 
            exp: String::new() 
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum JsonToken {
    BeginArray,
    BeginObject,
    EndArray,
    EndObject,
    NameSeparator,
    ValueSeparator,
    Digit(char),
    Number(JsonNumberToken),
    DecimalPoint,
    WhiteSpace(char),
    Exponent,
    Minus,
    Plus,
    True,
    False,
    Null,
    QuotationMark,
    String(Vec<JsonCharToken>)
}

#[derive(Debug)]
pub struct JsonTokens {
    pub tokens: Vec<JsonToken>
}

impl JsonTokens {
    pub fn ignore_whitespace(self) -> JsonTokens {
        let mut tokens = Vec::with_capacity(self.tokens.len());
        for token in self.tokens {
            match token {
                JsonToken::WhiteSpace(_) => {},
                _ => tokens.push(token),
            }
            
        }
        JsonTokens { tokens }
    }
}

impl IntoIterator for JsonTokens {
    type Item = JsonToken;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

#[derive(Debug)]
pub struct JsonLexerError {
    pub(crate) message: String
}

impl JsonLexerError {
    pub fn new(message : String) -> JsonLexerError {
        JsonLexerError { message }
    }
}

pub struct JsonLexer<'a> {
    json_chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl JsonLexer<'_> {
    pub fn new(json_string: &str) -> JsonLexer {
        JsonLexer { json_chars: json_string.chars().peekable() }
    }
    
    pub fn tokenize(&mut self) -> Result<JsonTokens, JsonLexerError> {
        
        let mut tokens = Vec::new();
        
        while let Some(token) = self.next_token()? {
            match token {
                JsonToken::Digit(d) => {
                    let mut num = JsonNumberToken::new(false);
                    num.integer.push(d);
                    let next_token = self.number_token(&mut num)?;
                    tokens.push(JsonToken::Number(num));
                    if let Some(token) = next_token {
                        tokens.push(token);
                    }
                },
                JsonToken::Minus => {
                    if let Some(JsonToken::Digit(d)) = self.next_token()? {
                        let mut num = JsonNumberToken::new(true);
                        num.integer.push(d);

                        let next_token = self.number_token(&mut num)?;
                        tokens.push(JsonToken::Number(num));
                        if let Some(token) = next_token {
                            tokens.push(token);
                        }
                    } else {
                        return Err(JsonLexerError::new("error: invalid token".to_string()));
                    }
                    
                },
                _ => tokens.push(token),
            }
        }

        Ok(JsonTokens { tokens })
    }

    fn next_return_token(&mut self, token: JsonToken) -> Option<JsonToken> {
        self.json_chars.next();
        Some(token)
    }
    
    fn next_token(&mut self) -> Result<Option<JsonToken>, JsonLexerError> {
        match self.json_chars.peek() {
            Some(&c) => match c {
                ' ' | '\n' | '\t' | '\r' => {
                    Ok(self.next_return_token(JsonToken::WhiteSpace(c)))
                },
                '{' => {
                    Ok(self.next_return_token(JsonToken::BeginObject))
                },
                '}' => {
                    Ok(self.next_return_token(JsonToken::EndObject))
                },
                '[' => {
                    Ok(self.next_return_token(JsonToken::BeginArray))
                },
                ']' => {
                    Ok(self.next_return_token(JsonToken::EndArray))
                },
                ':' => {
                    Ok(self.next_return_token(JsonToken::NameSeparator))
                },
                ',' => {
                    Ok(self.next_return_token(JsonToken::ValueSeparator))
                },
                '\"' => {
                    self.json_chars.next();
                    self.string_token()
                },
                '-' => {
                    Ok(self.next_return_token(JsonToken::Minus))
                },
                '+' => {
                    Ok(self.next_return_token(JsonToken::Plus))
                },
                '.' => {
                    Ok(self.next_return_token(JsonToken::DecimalPoint))
                },
                'e' | 'E' => {
                    Ok(self.next_return_token(JsonToken::Exponent))
                },
                '0'..='9' => {
                    Ok(self.next_return_token(JsonToken::Digit(c)))
                },
                't' => {
                    if self.json_chars.by_ref().take(4).eq(['t','r','u','e']) {
                        Ok(Some(JsonToken::True))
                    } else {
                        Err(JsonLexerError::new("error: found invalid value, expect \"true\"".to_string()))
                    }
                },
                'f' => {
                    if self.json_chars.by_ref().take(5).eq(['f','a','l','s','e']) {
                        Ok(Some(JsonToken::False))
                    } else {
                        Err(JsonLexerError::new("error: found invalid value, expect \"false\"".to_string()))
                    }
                },
                'n' => {
                    if self.json_chars.by_ref().take(4).eq(['n','u','l','l']) {
                        Ok(Some(JsonToken::Null))
                    } else {
                        Err(JsonLexerError::new("error: found invalid value, expect \"null\"".to_string()))
                    }
                },
                _ => {
                    Err(JsonLexerError::new(format!("error: found invalid value '{}'", c as char)))
                }
            },
            None => {
                Ok(None)
            }
        }
    }

    fn string_token(&mut self) -> Result<Option<JsonToken>, JsonLexerError> {
        let mut chars = Vec::new();
        while let Some(c) = self.json_chars.next() {
            match c {
                '\"' => {
                    return Ok(Some(JsonToken::String(chars)));
                },
                '\u{20}'..='\u{21}' | '\u{23}'..='\u{5b}' | '\u{5d}'..='\u{10ffff}' => {
                    chars.push(JsonCharToken::UnEscaped(c.to_string()));
                },
                '\\' => {
                    match self.json_chars.next() {
                        Some(escaped) => {
                            match escaped {
                                't' => chars.push(JsonCharToken::Escape("\t".to_string())),
                                'r' => chars.push(JsonCharToken::Escape("\r".to_string())),
                                'n' => chars.push(JsonCharToken::Escape("\n".to_string())),
                                '\\' => chars.push(JsonCharToken::Escape("\\".to_string())),
                                '\"' => chars.push(JsonCharToken::Escape("\"".to_string())), 
                                '/' => chars.push(JsonCharToken::Escape("/".to_string())),
                                'u' => {
                                    match u16::from_str_radix(&String::from_iter(self.json_chars.by_ref().take(4)), 16) {
                                        Ok(code) => {
                                            chars.push(JsonCharToken::Unicode(code));
                                        },
                                        Err(e) => {
                                            return Err(JsonLexerError::new(e.to_string()));
                                        }
                                    }
                                     
                                }
                                _ => return Err(JsonLexerError::new(format!("error: invalid char {}", c))),
                            }
                        },
                        None => {
                            return Err(JsonLexerError::new("error: end ob file, expect any escape char".to_string()));
                        }
                    } 
                },
                _ => {
                    return Err(JsonLexerError::new(format!("error: invalid char {}", c)));
                }
            }
        }
        Err(JsonLexerError::new("error: can not find close quotation-mark".to_string()))
    }

    fn number_token(&mut self, number : &mut JsonNumberToken) -> Result<Option<JsonToken>, JsonLexerError> {
        while let Some(token) = self.next_token()? {
            match token {
                JsonToken::Digit(d) => {
                    number.integer.push(d);
                },
                JsonToken::DecimalPoint =>{
                    if let Some(JsonToken::Digit(d)) = self.next_token()? {
                        number.frac.push(d);
                        while let Some(token) = self.next_token()? {
                            match token {
                                JsonToken::Digit(d) => {
                                    number.frac.push(d);
                                },
                                JsonToken::Exponent => {
                                    let sign = self.next_token()?;
                                    if let Some(JsonToken::Minus) = sign {
                                        number.exp.push('-');
                                    } else if let Some(JsonToken::Plus) = sign {
                                        number.exp.push('+');
                                    } else {
                                        return Err(JsonLexerError::new("error: exponent part must contain plus or minus.".to_string()));
                                    }
                                    while let Some(token) = self.next_token()? {
                                        match token {
                                            JsonToken::Digit(d) => number.exp.push(d),
                                            _=> {
                                                return Ok(Some(token));
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    return Ok(Some(token));
                                }
                            }
                        }
                        break;
                    } else {
                        return Err(JsonLexerError::new("error: fraction part must contain one or more digit.".to_string()));
                    }
                },
                JsonToken::Exponent =>{
                    let sign = self.next_token()?;
                    if let Some(JsonToken::Minus) = sign {
                        number.exp.push('-');
                    } else if let Some(JsonToken::Plus) = sign {
                        number.exp.push('+');
                    } else {
                        return Err(JsonLexerError::new("error: exponent part must contain plus or minus.".to_string()));
                    }
                    while let Some(token) = self.next_token()? {
                        match token {
                            JsonToken::Digit(d) => number.exp.push(d),
                            _=> {
                                return Ok(Some(token));
                            }
                        }
                    }
                    break;
                },
                _ => { 
                    return Ok(Some(token));
                }
            }
            
        }
        Ok(None)
    }
}