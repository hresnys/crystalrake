pub mod json;
mod error;
mod parser;
mod lexer;

#[test]
fn tokenize_null() {
    use lexer::*;
    let mut lexer = JsonLexer::new("null");
    match lexer.tokenize() {
        Ok(tokens) => {
            let v = Vec::from([JsonToken::Null]);
            assert!(v.iter().eq(tokens.tokens.iter()), "{:?}", tokens.tokens);
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}