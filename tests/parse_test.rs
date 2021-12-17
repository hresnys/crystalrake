extern crate crystalrake;
use crystalrake::json::*;

#[test]
fn true_value() {   
    let t = "true".parse::<JsonValue>();
    if let Ok(JsonValue::Boolean(b)) = t {
        assert!(b, "expect true, but {}", b);
    } else {
        panic!("unexpect value : {:?}", t);
    }
}
#[test]
fn true_string() {   
    let t = "\"true\"".parse::<JsonValue>().unwrap();
    assert_eq!(t, JsonValue::String("true".to_string()));
}

#[test]
fn false_value() {
    let f = "false".parse::<JsonValue>();
    if let Ok(JsonValue::Boolean(b)) = f {
        assert!(!b, "expect false, but {}", b);
    } else {
        panic!("unexpect value : {:?}", f);
    }
}

#[test]
fn false_string() {   
    let t = "\"false\"".parse::<JsonValue>().unwrap();
    assert_eq!(t, JsonValue::String("false".to_string()));
}

#[test]
fn number_value() {
    let json_value = "1234567890.0987654321".parse::<JsonValue>();
    if let Ok(JsonValue::Number(number)) = json_value {
        let integer = number as u64;
        assert_eq!(integer, 1234567890);
        assert_eq!(number.fract(),0.0987654321f64);
    } else {
        panic!("unexpect value : {:?}", json_value);
    }
}

#[test]
fn number_string() {   
    let t = "\"1234567890.0987654321\"".parse::<JsonValue>().unwrap();
    assert_eq!(t, JsonValue::String("1234567890.0987654321".to_string()));
}

#[test]
fn empty_object() {
    let json_value = "{}".parse::<JsonValue>().unwrap();
    let expect_result = JsonValue::Objects(Vec::new());
    assert_eq!(json_value, expect_result);
}

#[test]
fn array_value() {
    let json_value = r#" [ 12345, true, false, null, "Hello, world", { "object" : {} } ]"#.parse::<JsonValue>().unwrap();
    let object : JsonObject = JsonObject::new("object", JsonValue::Objects(Vec::new()));
    let values = Vec::from([
        JsonValue::Number(12345f64), 
        JsonValue::Boolean(true), 
        JsonValue::Boolean(false), 
        JsonValue::Null, 
        JsonValue::String("Hello, world".to_string()), 
        object.into()]);
    assert_eq!(JsonValue::Array(values), json_value);
}

#[test]
fn one_object() {
    let json_value = r#"{"name" : 12345}"#.parse::<JsonValue>().unwrap();
    let expect_result = JsonValue::Objects(Vec::from([JsonObject::new("name", 12345f64)]));
    assert_eq!(json_value, expect_result);
}

#[test]
fn nested_empty_object() {
    let json_value = r#"{ "empty" : {} }"#.parse::<JsonValue>().unwrap();
    let expect_result = JsonValue::Objects(Vec::from([JsonObject::new("empty", JsonValue::Objects(Vec::new()))]));
    assert_eq!(json_value, expect_result);
}

#[test]
fn nested_empty_array() {
    let json_value = r#"[ {"empty" : []} ]"#.parse::<JsonValue>().unwrap();
    let expect_result = JsonValue::Array(vec![JsonValue::Objects(vec![JsonObject::new("empty", JsonValue::Array(Vec::new()))])]);
    assert_eq!(json_value, expect_result);
}

#[test]
fn null_value() {
    let json_value = "null".parse::<JsonValue>();
    if let Ok(v) = json_value {
        assert!(v.is_null());
    } else {
        panic!("unexpect value : {:?}", json_value);
    }
}

#[test]
fn null_string() {
    let json_value = "\"null\"".parse::<JsonValue>().unwrap();
    assert_eq!(json_value, JsonValue::String("null".to_string()));
}

#[test]
fn contain_utf16() {
    let json_value = r#""\u3042\u3044\u3046abc""#.parse::<JsonValue>();
    if let Ok(JsonValue::String(v)) = json_value {
        assert_eq!(v, "あいうabc".to_string());
    } else {
        panic!("unexpect value : {:?}", json_value);
    }
}

#[test]
fn contain_emoji() {
    let json_value = r#""\uD83D\uDE04\uD83D\uDE07\uD83D\uDC7A""#.parse::<JsonValue>();
    if let Ok(JsonValue::String(v)) = json_value {
        assert_eq!(v, r#"😄😇👺"#.to_string());
    } else {
        panic!("unexpect value : {:?}", json_value);
    }
}