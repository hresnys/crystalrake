extern crate crystalrake;

use crystalrake::json::*;

#[derive(Debug, Eq, PartialEq)]
struct A {
    number : i32
}

impl A {
    fn new(number : i32) -> A {
        A { number }
    }
}

#[derive(Debug)]
struct JsonDeserializeError {
    message: String,
}

impl FromJson for A {
    type Err = JsonDeserializeError;
    fn from_json(json: &JsonValue) -> Result<Self, Self::Err> {
        match json {
            // This code must change. It's panicable.
            JsonValue::Number(n) =>  Ok(A { number: *n as i32}) ,
            _ => Err(JsonDeserializeError{ message: "fail".to_owned()})
        }
    }
}

#[test]
#[allow(overflowing_literals)]
fn simple_deserialize() {
    let a : A = "1000000000000000".parse::<JsonValue>().unwrap().deserialize().unwrap();
    assert_eq!(a, A::new(1000000000000000));
}