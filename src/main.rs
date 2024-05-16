use std::fs::File;
use std::io::Read;
use std::iter::Peekable;
use std::slice::Iter;
use std::str::Chars;

use lexer::{Token, TokenType};
use runny::run;

mod lexer;
mod runny;
mod tokenizer;
mod types;

fn main() {
    unsafe {
        run(Vec::from([
            TokenType::Identifier(String::from("sad")),
            TokenType::Symbol(String::from("=")),
            TokenType::Number(String::from("3")),
            TokenType::Symbol(String::from("+")),
            TokenType::GroupOpen(String::from("(")),
            TokenType::Number(String::from("7")),
            TokenType::Symbol(String::from("*")),
            TokenType::Number(String::from("4")),
            TokenType::GroupClose(String::from(")")),
            TokenType::LineFeed,
            TokenType::LineFeed,
            TokenType::LineFeed,
            TokenType::LineFeed,
        ]));
    }
}

fn spaces_to_tabs(code: &mut str) -> String {
    let mut thingy: Vec<char> = code.chars().collect();
    let mut code_iter: Peekable<Iter<char>> = thingy.iter().peekable();
    let mut num_spaces: i32 = 0;

    while let Some(c) = code_iter.next() {
        match c {
            '\n' => {
                if *code_iter.peek().unwrap() == &' ' {
                    while code_iter.next().unwrap() == &' ' {
                        num_spaces += 1;
                    }
                    break;
                }
            }
            _ => (),
        }
    }

    let mut counting: bool = false;
    let mut num: i32 = 0;
    thingy.retain(|x: &char| match x {
        ' ' if counting => {
            if num == num_spaces - 1 {
                num = 0;
                true
            } else {
                num += 1;
                false
            }
        }
        '\n' => {
            counting = true;
            num = 0;
            true
        }
        _ => {
            counting = false;
            true
        }
    });

    for i in 0..thingy.len() - 1 {
        match thingy[i] {
            '\n' if thingy[i + 1] == ' ' => {
                let mut j: usize = i + 1;
                while thingy[j] == ' ' {
                    thingy[j] = '\t';
                    j += 1;
                }
            }
            _ => (),
        };
    }

    let a: String = thingy.iter().collect();
    return a;
}

fn read_file(path: &str) -> String {
    let mut yes: File = File::open(path).expect("REASON");
    let mut contents = String::new();
    let _ = yes.read_to_string(&mut contents);
    return contents;
}
