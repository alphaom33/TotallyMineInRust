use regex::Regex;
use regex::Match;

pub static mut ST: String = String::new();

static SYMBOLS: &str = "+\\-*/\\^%=><\\\\&|~;:,.!?'\\\"`@";
static NO_NAME: &str = "()\\[\\]{}#\n";
static COMMENT_REGEX: &str = "(##(#?[^#])*##)|(#[^\n]*\n)";
static SPACE_REGEX: &str = r"\s+";
static STRING_REGEX: &str = "`[^`]*`";
static DEC_REGEX: &str = r"(\+|-)?(\d+(_(\d+))*)?\.?(\d+(_(\d+))*)+i?(e|E(\+|-)?\d+)?";
static NUM_REGEX: &str = r"(\+|-)?((0b[01]*\.?[01]+)|(0o[0-7]*\.?[0-7]+)|(0x[0-9a-fA-F]*\.?[0-9a-fA-F]+))i?";


pub fn evaluate(code: &str) {
  let mut st: &str = code;
  while !st.is_empty() {
    let ind: usize = get_next(st);
    let token: &str = &st[..ind];

    println!("{}", token);
    st = &st[ind..];
  }
}


pub fn match_quote(code: &str) -> usize {
  let quote: char = code.chars().nth(0).unwrap();
  let mut i: usize = 0;
  let mut back: bool = false;
  
  while i < code.len() - 1 {
    i += 1;
    let current: char = code.chars().nth(i).unwrap();
    if back {
      back = false;
      continue;
    }
    if current == quote {
      break;
    } if current == '\\' {
      back = true;
    } else if current == '{' {
      i += match_group(&code[i..], '}');
    }
  }
  return i;
}


pub fn match_group(code: &str, close: char) -> usize {
  let open: char = code.chars().nth(0).unwrap();
  let mut i: usize = 0;
  let mut balance: i32 = 0;
  
  while i < code.len() {
    let current: char = code.chars().nth(i).unwrap();
    let prev: Option<char> = code.chars().nth(i);

    // if you can improve this please do
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
}


pub fn get_next(code: &str) -> usize {
  if code.starts_with("\"") || code.starts_with("'") {
    return 1 + match_quote(code);
  } else if code.starts_with("(") {
    return 1 + match_group(code, ')');
  } else if code.starts_with("[") {
    return 1 + match_group(code, ']')
  } else if code.starts_with("{") {
    return 1 + match_group(code, '}')
  }
  
  let identifier_regex: &str = &format!(r"[^\d\s{}{}][^\s{}{}]*", SYMBOLS, NO_NAME, SYMBOLS, NO_NAME);
  let symbol_regex: &str = &format!("[{}]+", SYMBOLS);
  
  let token_regex: Regex = Regex::new(&format!("^({})|({})|({})|({})|({})|({})|({})", COMMENT_REGEX, SPACE_REGEX, DEC_REGEX, NUM_REGEX, STRING_REGEX, symbol_regex, identifier_regex)).unwrap();

  let token: Option<Match> = Regex::find(&token_regex, code);
  if !token.is_none() {
    return token.unwrap().end();
  } return code.len();
}

pub fn set_ST(code: String) {
  unsafe {
    ST = code;
  }
}

pub fn ST_empty() -> bool {
  unsafe {
    return ST.is_empty();
  }
}

pub fn advance() {
  unsafe {
    let ind: usize = get_next(ST.as_str());
    let token: &str = &ST[..ind];

    println!("{}", token);
    ST = ST[ind..].to_string();
  }
}