use std::io::Read;
use std::fs::File;
use regex::Regex;

mod interpreter;

fn main() {
    println!("{}", read_file("code/main.dym"));
}

fn read_file(path: &str) -> String {
        let mut yes: File = File::open(path).expect("REASON");
        let mut contents = String::new();
        let _ = yes.read_to_string(&mut contents);
        return contents;
}