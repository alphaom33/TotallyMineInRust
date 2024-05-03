use std::io::Read;
use std::fs::File;

mod lexer;

fn main() {
  let path: &str = "code/main.dym";
  let code: &str = &read_file(path);
  lexer::evaluate(code);
  // but the print statement stuff is just for testing
  // like your advance function

  lexer::set_st(code.to_string());
  while !lexer::st_empty() {
    lexer::advance();
  }
}


fn read_file(path: &str) -> String {
  let mut yes: File = File::open(path).expect("REASON");
  let mut contents = String::new();
  let _ = yes.read_to_string(&mut contents);
  return contents;
}