use crate::lexer::{lex, Token, TokenType};
use crate::post_processor::post_process;
use crate::read_file;

#[test]
fn lex_test() {
    let path: &str = "test/lex.dym";
    let mut tokens: Vec<Token> = lex(&read_file(path));
    assert_eq!(
        tokens.iter().map(|x: &Token| &x.token).filter(|x| return match x {
            TokenType::Whitespace(_) => false,
            _ => true,
        }).collect::<Vec<&TokenType>>(),
        [
            &TokenType::GroupOpen(String::from("{")),
            &TokenType::GroupClose(String::from("}")),
            &TokenType::GroupOpen(String::from("(")),
            &TokenType::GroupClose(String::from(")")),
            &TokenType::GroupOpen(String::from("[")),
            &TokenType::GroupOpen(String::from("]")),
            &TokenType::Symbol(String::from("=")),
            &TokenType::Symbol(String::from("->")),
            &TokenType::Symbol(String::from("?=")),
            &TokenType::Symbol(String::from(">=")),
            &TokenType::Symbol(String::from(">")),
            &TokenType::Symbol(String::from("<=")),
            &TokenType::Symbol(String::from("<")),
            &TokenType::Symbol(String::from("!=")),
            &TokenType::Symbol(String::from("!")),
            &TokenType::LineFeed,
            &TokenType::Symbol(String::from(",")),
            &TokenType::String(String::from("()")),
            &TokenType::String(String::from("[")),
            &TokenType::Symbol(String::from("+")),
            &TokenType::Symbol(String::from("-")),
            &TokenType::Symbol(String::from("*")),
            &TokenType::Symbol(String::from("/")),
            &TokenType::Symbol(String::from("&&")),
            &TokenType::Symbol(String::from("&")),
            &TokenType::Symbol(String::from("||")),
            &TokenType::Symbol(String::from("|")),
            &TokenType::Symbol(String::from("^")),
            &TokenType::DecimalNum(String::from("1234")),
            &TokenType::Number(String::from("01101")),
            &TokenType::Number(String::from("f32a")),
            &TokenType::Identifier(String::from("name😄"))
        ]
    );
}