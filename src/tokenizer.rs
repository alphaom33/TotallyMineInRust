use std::{fmt, iter::Peekable, str::Chars};

// pub static mut TOKENS: Vec<TokenTypes> = Vec::new();

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    LeftFuncParenthesis,
    RightFuncParenthesis,
    LeftCallParenthesis,
    RightCallParenthesis,
    LeftCaptureParenthesis,
    RightCaptureParenthesis,
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
    Lambda,
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Xor,
    Not,
    UnaryAnd,
    UnaryOr,
    None,
    Tab,
    Colon,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn lex(code: &str) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let mut code_iter: Peekable<Chars> = code.chars().into_iter().peekable();
    let mut i: usize = 0;
    while let Some(c) = code_iter.next() {
        tokens.push(match_basic(c, &mut code_iter, i, code));
        i += 1;
    }
    return tokens;
}

fn match_basic(c: char, code_iter: &mut Peekable<Chars>, i: usize, code: &str) -> TokenType {
    match c {
        '(' => TokenType::LeftParenthesis,
        ')' => TokenType::RightParenthesis,
        '{' => TokenType::LeftCurlyBracket,
        '}' => TokenType::RightCurlyBracket,
        '[' => TokenType::LeftBracket,
        ']' => TokenType::RightBracket,
        '=' => match code_iter.peek().unwrap() {
            '>' => {
                code_iter.next();
                TokenType::Lambda
            }
            _ => TokenType::Set,
        },
        '?' => {
            code_iter.next();
            TokenType::Equals
        }
        '>' => match code_iter.peek().unwrap() {
            '=' => {
                code_iter.next();
                TokenType::GreaterThanOrEqualTo
            }
            _ => TokenType::GreaterThan,
        },
        '<' => match code_iter.peek().unwrap() {
            '=' => {
                code_iter.next();
                TokenType::LessThanOrEqualTo
            }
            _ => TokenType::LessThan,
        },
        '!' => match code_iter.peek().unwrap() {
            '=' => {
                code_iter.next();
                TokenType::NotEquals
            }
            _ => TokenType::Not,
        },
        '#' => {
            while code_iter.peek().unwrap() != &'\n' {
                code_iter.next();
            }
            TokenType::None
        }
        '\n' | ';' => TokenType::LineFeed,
        ',' => TokenType::Comma,
        '"' | '\'' => {
            let mut accumulator: String = String::new();
            while code_iter.peek().unwrap() != &c {
                accumulator += &code_iter.next().unwrap().to_string();
            }
            code_iter.next();
            TokenType::String(if accumulator.len() > 0 {
                accumulator
            } else {
                String::new()
            })
        }
        '+' => TokenType::Add,
        '-' => TokenType::Subtract,
        '*' => TokenType::Multiply,
        '/' => TokenType::Divide,
        '&' => match code_iter.peek().unwrap() {
            '&' => {
                code_iter.next();
                TokenType::And
            }
            _ => TokenType::UnaryAnd,
        },
        '|' => match code_iter.peek().unwrap() {
            '|' => {
                code_iter.next();
                TokenType::Or
            }
            _ => TokenType::UnaryOr,
        },
        '^' => TokenType::Xor,
        ' ' | '\r' => TokenType::None,
        ':' => TokenType::Colon,
        '\t' => TokenType::Tab,
        _ if c.to_digit(10).is_some() => {
            let mut accumulator: String = c.to_string();

            let radix: u32 = match code_iter.peek().unwrap() {
                'x' => {
                    accumulator = String::new();
                    code_iter.next();
                    16
                }
                'b' => {
                    accumulator = String::new();
                    code_iter.next();
                    2
                }
                _ => 10,
            };

            while code_iter.peek().unwrap().to_digit(radix).is_some() {
                accumulator += &code_iter.next().unwrap().to_string();
            }
            TokenType::Number(accumulator)
        }
        _ => {
            let mut accumulator: String = c.to_string();
            'notcleanup: {
                if code_iter.peek().is_none() {
                    break 'notcleanup;
                }
                while ![
                    '=', ';', '?', '!', '>', '<', ',', '(', ')', '{', '}', '[', ']', ' ', '\n', '\r',
                ]
                .contains(code_iter.peek().unwrap())
                {
                    accumulator += &code_iter.peek().unwrap().to_string();
                    code_iter.next();
                    if code_iter.peek().is_none() {
                        break 'notcleanup;
                    }
                }
            }
            TokenType::Name(accumulator)
        }
    }
}