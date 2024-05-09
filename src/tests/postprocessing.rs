use crate::{lexer::{lex, Token, TokenType}, post_processor::post_process, read_file};

#[test]
fn post_process_test() {
    let path: &str = "test/postprocess.dym";
    let lexed: &mut Vec<Token> = &mut lex(&read_file(path));
    post_process(lexed);
    assert_eq!(
        *lexed,
        [
            TokenType::Identifier(String::from("foo")),
            TokenType::Symbol(String::from("=")),
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
            TokenType::Name(String::from("barp")),
            TokenType::Set,
            TokenType::LeftFuncParenthesis,
            TokenType::RightFuncParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::RightCurlyBracket,
        ]
    )
}

