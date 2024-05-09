use std::vec::Vec;
use regex::Regex;
use regex::Match;

static IDENTIFIER_REGEX: &str = "[^\\d\\s+\\-*/\\^%=><\\\\&|~;:,.!?'\"`$@()\\[\\]{}#][^\\s+\\-*/\\^%=><\\\\&|~;:,.!?'\"`$@()\\[\\]{}#]*";
static COMMENT_REGEX: &str = "(##(#?[^#])*##)|(#[^\n]*\n)";
static SPACE_REGEX: &str = "([\\s^\n]|\n)+";
static STRING_REGEX: &str = "\\A`[^`]*`|\\'(\\\\.|[^\\\\'])*\\'|\\\"(\\\\.|[^\\\\\"])*\\\"";
static SYMBOL_REGEX: &str = r"->|<?\.\.<?|[?+\-:]{2}|([+\-*/%&|\^?!><]|[/*><]{2})=?|[$@=;:,.~()\[\]{}]";
static DEC_REGEX: &str = r"(\d+(_(\d+))*)?\.?(\d+(_(\d+))*)+i?(e|E(\+|-)?\d+)?";
static NUM_REGEX: &str = r"((0b[01]*\.?[01]+)|(0o[0-7]*\.?[0-7]+)|(0x[0-9a-fA-F]*\.?[0-9a-fA-F]+))i?";

pub fn matches_regex(regex: &str, token: &str) -> bool {
  return Regex::is_match(&Regex::new(regex).unwrap(), &token);
}

enum TokenType {
  Identifier,
  DecimalNum,
  Number,
  String,
  GroupOpen,
  GroupClose,
  Symbol,
  Whitespace,
  LineFeed,
  Comment
}

pub struct Token {
  token: TokenType,
  pub string: String
}

fn token_type(token: &str) -> TokenType {
  if token == "\n" {
    return TokenType::LineFeed;
  } else if token.starts_with("#") {
    return TokenType::Comment;
  } else if "([{".contains(&token) {
    return TokenType::GroupOpen;
  } else if ")]}".contains(&token) {
    return TokenType::GroupClose;
  } else if token.trim().is_empty() {
    return TokenType::Whitespace;
  } else if matches_regex(STRING_REGEX, &token) {
    return TokenType::String;
  } else if matches_regex(DEC_REGEX, &token) {
    return TokenType::DecimalNum;
  } else if matches_regex(NUM_REGEX, &token) {
    return TokenType::Number;
  } else if matches_regex(SYMBOL_REGEX, &token) {
    return TokenType::Symbol;
  } return TokenType::Identifier;
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
    return Token {
      token: token_type(string),
      string: string.to_string()
    };
  } panic!("SyntaxError: invalid token, starting at:\n{}", code);
}

pub fn lex(mut code: &str) -> Vec<Token> {
  let mut output: Vec<Token> = Vec::new();
  while !code.is_empty() {
    let current: Token = get_next(code);
    code = &code[current.string.len()..];
    output.push(current);
  }
  return output;
}
