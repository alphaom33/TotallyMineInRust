use crate::lexer::{lex, TokenType};
use crate::post_processor::post_process;
use crate::read_file;

#[test]
fn lex_test() {
    let path: &str = "test/lex.dym";
    assert_eq!(
        lex(&read_file(path)),
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

#[test]
fn post_process_test() {
    let path: &str = "test/postprocess.dym";
    let lexed: &mut Vec<TokenType> = &mut lex(&read_file(path));
    post_process(lexed);
    assert_eq!(
        *lexed,
        [
            TokenType::Name(String::from("foo")),
            TokenType::Set,
            TokenType::LeftFuncParenthesis,
            TokenType::RightFuncParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::LineFeed,
            TokenType::Name(String::from("bar")),
            TokenType::Set,
            TokenType::LeftFuncParenthesis,
            TokenType::RightFuncParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::LeftCaptureParenthesis,
            TokenType::Name(String::from("baz")),
            TokenType::Add,
            TokenType::Name(String::from("bonk")),
            TokenType::RightCurlyBracket,
            TokenType::RightCaptureParenthesis,
            TokenType::LineFeed,
            TokenType::RightCurlyBracket,
            TokenType::LineFeed,
            TokenType::Name(String::from("bop")),
            TokenType::LeftCallParenthesis,
            TokenType::LeftFuncParenthesis,
            TokenType::Name(String::from("beep")),
            TokenType::RightFuncParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::Name(String::from("bope")),
            TokenType::RightCurlyBracket,
            TokenType::RightCallParenthesis,
            TokenType::LineFeed,
        ]
    )
}