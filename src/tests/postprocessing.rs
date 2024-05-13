use crate::{lexer::{lex, Token, TokenType}, post_processor::post_process, read_file};

#[test]
fn post_process_test() {
    let path: &str = "test/postprocess.dym";
    let lexed: &mut Vec<Token> = &mut lex(&read_file(path));
    post_process(lexed);
    assert_eq!(
        lexed.iter().map(|x| &x.token).collect::<Vec<&TokenType>>(),
        [
            &TokenType::Identifier(String::from("foo")),
            &TokenType::Symbol(String::from("=")),
            &TokenType::GroupOpen(String::from("(")),
            &TokenType::GroupOpen(String::from(")")),
            &TokenType::Symbol(String::from("->")),
            &TokenType::GroupOpen(String::from("{")),
            &TokenType::LineFeed,
            &TokenType::Identifier(String::from("bar")),
            &TokenType::Symbol(String::from("=")),
            &TokenType::GroupOpen(String::from("(")),
            &TokenType::GroupOpen(String::from(")")),
            &TokenType::Symbol(String::from("->")),
            &TokenType::GroupOpen(String::from("{")),
            &TokenType::GroupOpen(String::from("(")),
            &TokenType::Identifier(String::from("baz")),
            &TokenType::Symbol(String::from("+")),
            &TokenType::Identifier(String::from("bonk")),
            &TokenType::GroupClose(String::from("}")),
            &TokenType::GroupOpen(String::from(")")),
            &TokenType::LineFeed,
            &TokenType::GroupClose(String::from("}")),
            &TokenType::LineFeed,
            &TokenType::Identifier(String::from("bop")),
            &TokenType::GroupClose(String::from("(")),
            &TokenType::GroupClose(String::from(")")),
            &TokenType::Identifier(String::from("beep")),
            &TokenType::GroupClose(String::from(")")),
            &TokenType::Symbol(String::from("->")),
            &TokenType::GroupOpen(String::from("{")),
            &TokenType::Identifier(String::from("bope")),
            &TokenType::GroupClose(String::from("}")),
            &TokenType::GroupClose(String::from(")")),
            &TokenType::LineFeed,
            &TokenType::Identifier(String::from("barp")),
            &TokenType::Symbol(String::from("=")),
            &TokenType::GroupClose(String::from("(")),
            &TokenType::GroupClose(String::from(")")),
            &TokenType::Symbol(String::from("->")),
            &TokenType::GroupOpen(String::from("{")),
            &TokenType::GroupClose(String::from("}")),
        ]
    )
}

