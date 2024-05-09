use std::vec::Vec;

mod lexer;

pub fn preprocess(code: &Vec<lexer::Token>) -> Vec<lexer::Token> {
  let mut output: Vec<lexer::Token> = Vec::new();
  let mut prev: lexer::Token = lexer::Token {
    token: lexer::TokenType::LineFeed,
    string: "".to_string()
  };
  let mut i: usize = 0;
  
  while i < code.len() {
    let current = code[i];
    if current == lexer::TokenType::Comment {
      i += 1;
    } else if is_func_thingy(code[i..]) {
      output.push(current);
      output.push(lexer::Token {
        token: lexer::TokenType::Symbol,
        string: "=".to_string()
      });
    }
    else {
      output.push(current);
    }
  }
}


fn is_func_thingy(code: Vec<lexer::Token>) -> bool {}

fn is_lambda_thingy(code: Vec<lexer::Token>) -> bool {}