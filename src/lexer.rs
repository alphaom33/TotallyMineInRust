use std::vec::Vec;
use regex::Regex;
use regex::Match;

static IDENTIFIER_REGEX: &str = "[^\\d\\s+\\-*/\\^%=><\\\\&|~;:,.!?'\"`@()\\[\\]{}#][^\\s+\\-*/\\^%=><\\\\&|~;:,.!?'\"`@()\\[\\]{}#]*";
static COMMENT_REGEX: &str = "(##(#?[^#])*##)|(#[^\n]*\n)";
static SPACE_REGEX: &str = "([\\s^\n]|\n)+";
static STRING_REGEX: &str = "`[^`]*`|'(\\\\.|[^'])'|\"(\\\\.|[^\"])\"";
static SYMBOL_REGEX: &str = r"->|<?..<?|[?+\-:]{2}|([+\-*/%&|\^?!><]|[/*><]{2})=?|[$@=;:,.~()\[\]{}]";
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

struct Token {
  token: TokenType,
  string: String
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


/*pub fn match_quote(code: &str) -> usize {
  let quote: char = code.chars().nth(0).unwrap();
  let mut i: usize = 0;
  
  while i < code.len() - 1 {
    i += 1;
    let current: char = code.chars().nth(i).unwrap();
    if current == quote {
      break;
    } if current == '\\' {
      i += 1;
      continue;
    } else if current == '{' {
      i += match_group(&code[i..], '}');
    }
  }
  return i;
}*/
/*pub fn match_group(code: &str, close: char) -> usize {
  let open: char = code.chars().nth(0).unwrap();
  let mut i: usize = 0;
  let mut balance: i32 = 0;
  
  while i < code.len() {
    let current: char = code.chars().nth(i).unwrap();
    let prev: Option<char> = code.chars().nth(i);

    // TODO: improve this code if possible
    // probably would send calls to get_next
    // if not then i think the lexer is done for now
    if current == '`' {
      i += 1 + code[i+1..].find("`").unwrap();
    } else if current == '#' && !prev.is_none() && prev.unwrap() == '#' {
      i += 2 + code[i+1..].find("##").unwrap();
    } else if current == '#' {
      i += 1 + code[i+1..].find("\n").unwrap();
    }
    else if current == '"' || current == '\'' {
      i += match_quote(&code[i..]);
    } else if current == open {
      balance += 1;
    } else if current == close {
      balance -= 1;
    }
    if balance == 0 {
      return i;
    }
    i += 1;
  }
  return i;
}*/


// TODO someday: add support for nested formatted strings
// (Python didn't add this until ver 3.12 lol)
pub fn get_next(code: &str) -> Token {
  let token_regex: Regex = Regex::new(&format!("^({})|({})|({})|({})|({})|({})|({})", COMMENT_REGEX, SPACE_REGEX, DEC_REGEX, NUM_REGEX, STRING_REGEX, SYMBOL_REGEX, IDENTIFIER_REGEX)).unwrap();

  let token: Option<Match> = Regex::find(&token_regex, code);
  if !token.is_none() {
    let t: Match = token.unwrap();
    if t.start() != 0 {
      let string: &str = &code[..t.end()];
      return Token {
        token: token_type(string),
        string: string.to_string()
      };
    }
  } panic!("SyntaxError: invalid token");
}


/*pub fn get_block(code: &str) -> Block {
  let mut st: &str = code;
  let mut tokens: Vec<String> = Vec::new();
  let mut end: usize = 0;

  // TODO: fix indented code block implementation
  while !st.is_empty() {
    let ind: usize = get_next(st);
    let current: &str = &st[..ind];

    if end == 0 {
      // do stuff
    } else if current == "\n" {
      end += 1;
      break;
    } else if current.starts_with("{") && current.ends_with("}") {
      break;
    } else if current.starts_with("#") && current.ends_with("\n") {
      end += ind;
      break;
    } else if current.trim().is_empty() || current.starts_with("##") {
      end += ind;
      st = &st[ind..];
      continue;
    }

    tokens.push(current.to_string());
    end += ind;
    st = &st[ind..];
  }

  return Block {
    block: tokens,
    end: end
  };
}*/