use std::io::Read;
use std::fs::File;

use lexer::TokenTypes;
use postProcessor::post_process;

mod lexer;
mod runinifier;
mod postProcessor;

fn main() {
  let path: &str = "code/main.dym";
  let code: &str = &read_file(path);
  let mut tokens: Vec<TokenTypes> = lexer::lex(code);
  for token in &tokens {
    // println!("{}", token);
  }
  println!("---------------------------");
  post_process(&mut tokens);
  for token in &tokens {
    println!("{}", token);
  }

  // lexer::set_st(code.to_string());
  // while !lexer::st_empty() {
  //   runinifier::def_dynanite_proc(lexer::advance());
  // }
}


fn read_file(path: &str) -> String {
  let mut yes: File = File::open(path).expect("REASON");
  let mut contents = String::new();
  let _ = yes.read_to_string(&mut contents);
  return contents;
}