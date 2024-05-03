mod lexer;

fn defDynaniteProc(token: &str) {
  match token {


    "#" => while (token != "\n") {token = lexer::advance();},
    _ => println!("varname"),
  }
}