use crate::json::{JsonValue, JsonObject};
use crate::lexer::*;

#[derive(Debug)]
pub struct JsonParseError {
    pub(crate) message : String
}

impl JsonParseError {
    pub fn new(message : String) -> JsonParseError {
        JsonParseError { message }
    }
}

impl ToString for JsonParseError {
    fn to_string(&self) -> String {
        self.message.to_owned()
    }
}

pub struct JsonParser {
    tokens : Vec<JsonToken>,
    position : usize
}

impl JsonParser {
    pub fn new(tokens: JsonTokens) -> JsonParser {
        JsonParser { tokens : tokens.tokens, position: 0 }
    }

    fn peek(&self) -> Option<&JsonToken> {
        self.tokens.get(self.position)
    }

    fn next(&mut self) -> Option<&JsonToken> {
        let current = self.tokens.get(self.position);
        self.position += 1;
        current
    }

    // fn skip_whitespace(&mut self) {
    //     while let Some(token) = self.peek()  {
    //         match token {
    //             JsonToken::WhiteSpace(_) => {
    //                 self.next();
    //                 continue;
    //             },
    //             _ => {
    //                 break;
    //             }
    //         }
    //     }
    // }

    fn token_to_string(s : &Vec<JsonCharToken>) -> Result<Option<JsonValue>, JsonParseError> {
        let mut buf = String::new();
        let mut utf16  = Vec::new();
        for c in s {
            match c {
                JsonCharToken::Escape(c) | JsonCharToken::UnEscaped(c) =>{
                    if !utf16.is_empty() {
                        match String::from_utf16(&utf16) {
                            Ok(utf16_str) => {
                                buf.push_str(&utf16_str);
                                utf16.clear();
                            },
                            Err(e) => {
                                return Err(JsonParseError::new(e.to_string()));
                            }
                        }
                    } 
                    buf.push_str(c);
                },
                JsonCharToken::Unicode(c) => {
                    utf16.push(*c);
                }
            }
        }
        if !utf16.is_empty() {
            match String::from_utf16(&utf16) {
                Ok(utf16_str) => {
                    buf.push_str(&utf16_str);
                },
                Err(e) => {
                    return Err(JsonParseError::new(e.to_string()));
                }
            }
        }
        return Ok(Some(JsonValue::String(buf))); 
    }

    pub fn get_value(&mut self) -> Result<JsonValue, JsonParseError> {
        let mut ret = None;
        while let Some(value) = self.next_value()? {
            if ret.is_none() {
                ret = Some(value);
            } else {
                return Err(JsonParseError::new(format!("invalid JSON format. JSON must contain only one value. fount {:?}", value)));
            }
        }
        match ret {
            Some(v) => Ok(v),
            None => Err(JsonParseError::new("invalid JSON format".to_string()))
        }
    }

    fn next_value(&mut self) -> Result<Option<JsonValue>, JsonParseError> {
        while let Some(token) = self.next() {
            match token {
                JsonToken::BeginObject => {
                    let mut objects = Vec::new();
                    while let Some(token) = self.peek()  {
                        match token {
                            JsonToken::WhiteSpace(_) => {
                                self.next();
                                continue;
                            },
                            JsonToken::EndObject => {
                                self.next();
                                return Ok(Some(JsonValue::Objects(objects)));
                            },
                            _ => {
                                objects.push(self.get_object()?);
                                while let Some(token) = self.peek() {
                                    match token {
                                        JsonToken::WhiteSpace(_) => { 
                                            self.next();
                                            continue; 
                                        },
                                        JsonToken::ValueSeparator => {
                                            self.next();
                                            objects.push(self.get_object()?);
                                            
                                        },
                                        JsonToken::EndObject => {
                                            self.next();
                                            return Ok(Some(JsonValue::Objects(objects)));
                                        },
                                        _ => {
                                            return Err(JsonParseError::new(format!("error: invalid token {:?}, expect whitespace, value-separator or end-object\nobjects: {:?}", token, objects)));
                                        }
                                    }
                                }
                            }

                        }
                    }
                    return Ok(Some(JsonValue::Objects(objects)));
                },
                JsonToken::BeginArray => {
                    let mut values = Vec::new(); 
                    while let Some(token) = self.peek() {
                        match token {
                            JsonToken::WhiteSpace(_) => {
                                self.next();
                                continue;
                            },
                            JsonToken::EndArray => {
                                return Ok(Some(JsonValue::Array(values)));
                            },
                            _ => {
                                if let Some(value) = self.next_value()? {
                                    values.push(value);
                                } else {
                                    return Ok(Some(JsonValue::Array(values)));
                                }

                                while let Some(token) = self.peek() {
                                    match token {
                                        JsonToken::WhiteSpace(_) => {
                                            self.next();
                                            continue;
                                        },
                                        JsonToken::EndArray =>{
                                            self.next();
                                            return Ok(Some(JsonValue::Array(values)));
                                        },
                                        JsonToken::ValueSeparator => {
                                            self.next();
                                            if let Some(value) = self.next_value()? {
                                                values.push(value);
                                            } else {
                                                return Err(JsonParseError::new("error: not found next value.".to_string()));
                                            }
                                        },
                                        _ => {
                                            return Err(JsonParseError::new(format!("error: could not parse array. invalid token {:?}", token)));
                                        }
                                    }
                                }
                                return Err(JsonParseError::new("error: could not find end of array(']')".to_string()));
                            }
                        }
                    }
                    unreachable!()
                },
                JsonToken::WhiteSpace(_) => {
                    //self.next();
                    return self.next_value();
                },
                JsonToken::Number(number) => {
                    let number = number.to_string();
                    match number.parse() {
                        Ok(v) => {
                            //self.next();
                            return Ok(Some(JsonValue::Number(v)));
                        },
                        Err(e) => {
                            return Err(JsonParseError::new(e.to_string()));
                        }
                    }
                },
                JsonToken::True => {
                    return Ok(Some(JsonValue::Boolean(true)));
                },
                JsonToken::False => {
                    return Ok(Some(JsonValue::Boolean(false)));
                },
                JsonToken::Null => {
                    return Ok(Some(JsonValue::Null));
                },
                JsonToken::String(s) => {
                    return JsonParser::token_to_string(s); 
                },
                _ => {
                    return Err(JsonParseError::new(format!("error: invalid token {:?}, could not get value.", token)));
                }
            }
        }
        Ok(None)
    }

    fn get_object(&mut self) -> Result<JsonObject, JsonParseError> {
        while let Some(token) = self.peek() {
            match token {
                JsonToken::WhiteSpace(_) => {
                    self.next();
                    continue;
                },
                JsonToken::String(s) => {
                    if let Some(JsonValue::String(key)) = JsonParser::token_to_string(s)?{ 
                        self.next();
                        while let Some(token) = self.peek() {
                            match token {
                                JsonToken::WhiteSpace(_) => { 
                                    self.next();
                                    continue;
                                },
                                JsonToken::NameSeparator => {
                                    self.next();
                                    return Ok(JsonObject::new(&key, self.next_value()?));
                                },
                                _ => {
                                    return Err(JsonParseError::new(format!("error: invalid token :{:?}, expect whitespace or name-separator(',')", token)));
                                },
                            }
                        }
                    }
                    return Err(JsonParseError::new("error: could not find any object name.".to_string()));
                },
                _ => {
                    return Err(JsonParseError::new(format!("error: invalid token {:?}, expect whitespace or string", token)));
                }
            }
        }
        unreachable!()
    }
}