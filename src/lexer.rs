use std::vec::Vec;
use regex::Regex;
use regex::Match;

use crate::types::BaseValue;

static IDENTIFIER_REGEX: &str = "[^\\d\\s+\\-*/\\^%=><\\\\&|~;:,.!?'\"`$@()\\[\\]{}#][^\\s+\\-*/\\^%=><\\\\&|~;:,.!?'\"`$@()\\[\\]{}#]*";
static COMMENT_REGEX: &str = "(##(#?[^#])*##)|(#[^\n]*\n)";
static SPACE_REGEX: &str = "([\\s^\n]|\n)+";
static STRING_REGEX: &str = "`[^`]*`|\\'(\\\\.|[^\\\\'])*\\'|\\\"(\\\\.|[^\\\\\"])*\\\"";
static SYMBOL_REGEX: &str = r"->|<?\.\.<?|[?+\-:]{2}|([+\-*/%&|\^?!><]|[/*><]{2})=?|[$@=;:,.~()\[\]{}]";
static DEC_REGEX: &str = r"(\d+(_(\d+))*)?\.?(\d+(_(\d+))*)+i?(e|E(\+|-)?\d+)?";
static NUM_REGEX: &str = r"((0b[01]*\.?[01]+)|(0o[0-7]*\.?[0-7]+)|(0x[0-9a-fA-F]*\.?[0-9a-fA-F]+))i?";

pub fn matches_regex(regex: &str, token: &str) -> bool {
  return Regex::is_match(&Regex::new(regex).unwrap(), &token);
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
  Group(Vec<Token>),
  Identifier(String),
  DecimalNum(String),
  Number(String),
  String(String),
  GroupOpen(String),
  GroupClose(String),
  Symbol(String),
  Whitespace(String),
  Value(BaseValue),
  LineFeed,
  Comment
}

impl TokenType {
  pub fn get_self(&self) -> Option<String> {
    match self {
        TokenType::Identifier(g) => Some(g.to_string()),
        TokenType::DecimalNum(g) => Some(g.to_string()),
        TokenType::Number(g) => Some(g.to_string()),
        TokenType::String(g) => Some(g.to_string()),
        TokenType::GroupOpen(g) => Some(g.to_string()),
        TokenType::GroupClose(g) => Some(g.to_string()),
        TokenType::Symbol(g) => Some(g.to_string()),
        TokenType::Whitespace(g) => Some(g.to_string()),
        _ => None,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Token {
  pub token: TokenType,
  length: usize
}

fn create_token(token: &str) -> Token {
  let token_type: TokenType;
  if token == "\n" {
    token_type = TokenType::LineFeed;
  } else if token.starts_with("#") {
    token_type = TokenType::Comment;
  } else if "([{".contains(&token) {
    token_type = TokenType::GroupOpen(token.to_string());
  } else if ")]}".contains(&token) {
    token_type = TokenType::GroupClose(token.to_string());
  } else if token.trim().is_empty() {
    token_type = TokenType::Whitespace(token.to_string());
  } else if matches_regex(STRING_REGEX, &token) {
    token_type = TokenType::String(token.to_string());
  } else if matches_regex(DEC_REGEX, &token) {
    token_type = TokenType::DecimalNum(token.to_string());
  } else if matches_regex(NUM_REGEX, &token) {
    token_type = TokenType::Number(token.to_string());
  } else if matches_regex(SYMBOL_REGEX, &token) {
    token_type = TokenType::Symbol(token.to_string());
  } else {
    token_type = TokenType::Identifier(token.to_string());
  }

  return Token {
    token: token_type,
    length: token.len()
  };
}

struct Block {
  block: Vec<Token>,
  end: usize
}


// TODO someday: add support for nested formatted strings
// (Python didn't add this until ver 3.12 lol)
fn get_next(code: &str) -> Token {
  let token_regex: Regex = Regex::new(&format!(r"\A({})|({})|({})|({})|({})|({})|({})", COMMENT_REGEX, SPACE_REGEX, DEC_REGEX, NUM_REGEX, STRING_REGEX, SYMBOL_REGEX, IDENTIFIER_REGEX)).unwrap();

  let token: Option<Match> = Regex::find(&token_regex, code);
  if !token.is_none() {
    let t: Match = token.unwrap();
    let string: &str = &code[..t.end()];

    return create_token(string);
  } panic!("SyntaxError: invalid token, starting at:\n{}", code);
}

pub fn lex(mut code: &str) -> Vec<Token> {
  let mut output: Vec<Token> = Vec::new();
  while !code.is_empty() {
    let current: Token = get_next(code);
    code = &code[current.length..];
    output.push(current);
  }
  return output;
}


pub fn match_group(code: &Vec<Token>, mut i: usize) -> usize {
  let mut balance: usize = 0;
  
  while i < code.len() {
    let current: &Token = &code[i];
    match current.token {
      TokenType::GroupOpen(_) => balance += 1,
      TokenType::GroupClose(_) => balance -= 1,
      _ => (),
    }
    if balance == 0 {
      return i;
    }
    i += 1;
  }
  panic!("SyntaxError: unclosed grouping symbol");
}
