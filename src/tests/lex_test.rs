use crate::lexer::{lex, Token, TokenType};
use crate::post_processor::post_process;
use crate::read_file;

#[test]
fn lex_test() {
    let path: &str = "test/lex.dym";
    let mut tokens: Vec<Token> = lex(&read_file(path));
    assert_eq!(
        tokens,
        [
            TokenType::LeftCurlyBracket,
            TokenType::RightCurlyBracket,
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::LeftBracket,
            TokenType::RightBracket,
            TokenType::Set,
            TokenType::Lambda,
            TokenType::Equals,
            TokenType::GreaterThanOrEqualTo,
            TokenType::GreaterThan,
            TokenType::LessThanOrEqualTo,
            TokenType::LessThan,
            TokenType::NotEquals,
            TokenType::Not,
            TokenType::LineFeed,
            TokenType::Comma,
            TokenType::String(String::from("()")),
            TokenType::String(String::from("[")),
            TokenType::Add,
            TokenType::Subtract,
            TokenType::Multiply,
            TokenType::Divide,
            TokenType::And,
            TokenType::UnaryAnd,
            TokenType::Or,
            TokenType::UnaryOr,
            TokenType::Xor,
            TokenType::Number(String::from("1234")),
            TokenType::Number(String::from("01101")),
            TokenType::Number(String::from("f32a")),
            TokenType::Name(String::from("name😄"))
        ]
    );
}