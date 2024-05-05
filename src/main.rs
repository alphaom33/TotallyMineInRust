use std::io::Read;
use std::fs::File;

use lexer::TokenType;
use post_processor::post_process;

mod lexer;
mod runinifier;
mod post_processor;
#[cfg(test)]
mod tests;

fn main() {
  let path: &str = "code/main.dym";
  let code: &str = &read_file(path);
  let mut tokens: Vec<TokenType> = lexer::lex(code);

  post_process(&mut tokens);
}

pub fn read_file(path: &str) -> String {
  let mut yes: File = File::open(path).expect("REASON");
  let mut contents = String::new();
  let _ = yes.read_to_string(&mut contents);
  return contents;
}