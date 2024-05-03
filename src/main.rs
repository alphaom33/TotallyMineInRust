use std::io::Read;
use std::fs::File;

mod lexer;

//static mut ST: String = String::new();

fn main() {
  let path: &str = "code/main.dym";
  let code: &str = &read_file(path);
  lexer::evaluate(code);
  //let mut st: &str = code;

    lexer::set_ST(code.to_string());
  
    while !lexer::ST_empty() {
      lexer::advance();
    }
}


fn read_file(path: &str) -> String {
  let mut yes: File = File::open(path).expect("REASON");
  let mut contents = String::new();
  let _ = yes.read_to_string(&mut contents);
  return contents;
}