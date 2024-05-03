use std::fmt;
use std::ptr::addr_of;

// pub static mut TOKENS: Vec<TokenTypes> = Vec::new();

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenTypes {
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftBracket,
    RightBracket,
    String(String),
    Number(String),
    Comma,
    Name(String),
    Set,
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    LineFeed,
}

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

pub fn lex(code: &str) -> Vec<TokenTypes> {
  let mut tokens: Vec<TokenTypes> = Vec::new();
  let mut code_iter = code.chars().into_iter().peekable();
    while let Some(c) = code_iter.next() {
        tokens.push(match c {
            '(' => TokenTypes::RightParenthesis,
            ')' => TokenTypes::LeftParenthesis,
            '{' => TokenTypes::LeftCurlyBracket,
            '}' => TokenTypes::RightCurlyBracket,
            '[' => TokenTypes::LeftBracket,
            ']' => TokenTypes::RightBracket,
            '=' => TokenTypes::Set,
            '?' => {
                code_iter.next();
                TokenTypes::Equals
            }
            '>' => match code_iter.peek().unwrap() {
                '=' => {
                    code_iter.next();
                    TokenTypes::GreaterThanOrEqualTo
                }
                _ => TokenTypes::GreaterThan,
            },
            '<' => match code_iter.peek().unwrap() {
                '=' => {
                    code_iter.next();
                    TokenTypes::LessThanOrEqualTo
                }
                _ => TokenTypes::LessThan,
            },
            '!' => {
                code_iter.next();
                TokenTypes::NotEquals
            }
            '#' => {
                while code_iter.next().unwrap() != '\n' {}
                continue;
            }
            '\n' => TokenTypes::LineFeed,
            ' ' => continue,
            ',' => TokenTypes::Comma,
            '"' | '\'' => {
                let mut accumulator: String = code_iter.peek().unwrap().to_string();
                while *code_iter.peek().unwrap() != c {
                    accumulator += &code_iter.next().unwrap().to_string();
                }
                code_iter.next();
                TokenTypes::String(if accumulator.len() > 1 {
                    accumulator
                } else {
                    "".to_string()
                })
            }
            _ if c.to_digit(10).is_some() => TokenTypes::Number(c.to_string()),
            _ => {
                let mut accumulator: String = c.to_string();
                let mut namey: char = code_iter.next().unwrap();
                while ![
                    '=', '?', '!', '>', '<', ',', '(', ')', '{', '}', '[', ']', ' ', '\n',
                ]
                .contains(&namey)
                {
                    accumulator += &namey.to_string();
                    if code_iter.peek().is_none() {
                        break;
                    }

                    namey = code_iter.next().unwrap();
                }
                TokenTypes::Name(accumulator)
            }
        })
    }
    return tokens;
}
