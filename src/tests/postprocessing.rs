use crate::{post_processor::{change_python, kill_nulls, post_process, scan_lambda_curlies}, preprocess, read_file, tokenizer::{lex, TokenType}};

#[test]
fn lambda_curlies_test() {
    let path: &str = "test/posprocessing/lambdaCurlies.dym";
    let mut lexed = lex_file(path);
    kill_nulls(&mut lexed);
    scan_lambda_curlies(&mut lexed);
    assert_eq!(
        lexed,
        [
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::Name(String::from("foo")),
            TokenType::RightCurlyBracket,
            TokenType::LineFeed,
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::Name(String::from("bar")),
            TokenType::Name(String::from("bat")),
            TokenType::RightCurlyBracket,
            TokenType::LineFeed,
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::Name(String::from("beep")),
            TokenType::RightCurlyBracket,
            TokenType::LineFeed,
            TokenType::Name(String::from("bop")),
            TokenType::LeftParenthesis,
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::Name(String::from("bope")),
            TokenType::RightCurlyBracket,
            TokenType::Comma,
            TokenType::Name(String::from("bomp")),
            TokenType::RightParenthesis,
            TokenType::LineFeed,
        ]
    );
}

#[test]
fn python_process_test() {
    let path: &str = "test/posprocessing/python.dym";
    let mut lexed = lex_file(path);
    change_python(&mut lexed);
    assert_eq!(
        lexed,
        [
            TokenType::Name(String::from("foo")),
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::LeftCurlyBracket,
            TokenType::LineFeed,
            TokenType::Tab,
            TokenType::Name(String::from("bar")),
            TokenType::LineFeed,
            TokenType::Tab,
            TokenType::Name(String::from("bat")),
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::LeftCurlyBracket,
            TokenType::LineFeed,
            TokenType::RightCurlyBracket,
            TokenType::LineFeed,
            TokenType::Tab,
            TokenType::Name(String::from("bob")),
            TokenType::LineFeed,
            TokenType::RightCurlyBracket,
            TokenType::LineFeed,
            TokenType::Name(String::from("beep")),
            TokenType::LineFeed,
            TokenType::Name(String::from("bop")),
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::LeftCurlyBracket,
            TokenType::LineFeed,
            TokenType::Tab,
            TokenType::Name(String::from("boop")),
            TokenType::LineFeed,
            TokenType::RightCurlyBracket,
            TokenType::LineFeed,
        ]
    )
}

fn lex_file(file: &str) -> Vec<TokenType> {
    return lex(&preprocess(file));
}

#[test]
fn post_process_test() {
    let mut lexed: Vec<TokenType> = lex_file("test/postprocess.dym");
    // let path: &str = "test/postprocess.dym";
    // let lexed: &mut Vec<TokenType> = &mut lex(&read_file(path));
    post_process(&mut lexed);
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
            TokenType::Name(String::from("barp")),
            TokenType::Set,
            TokenType::LeftFuncParenthesis,
            TokenType::RightFuncParenthesis,
            TokenType::Lambda,
            TokenType::LeftCurlyBracket,
            TokenType::RightCurlyBracket,
            TokenType::LineFeed,
        ]
    )
}

