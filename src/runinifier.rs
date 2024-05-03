// use crate::lexer::advance;

pub fn def_dynanite_proc(token: String) {
  match token.as_str() {
    // "#" => while token != "\n" {token = advance();},
    _ => println!("{}", token.as_str()),
  }
}