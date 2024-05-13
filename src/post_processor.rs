use std::{iter::{Enumerate, Skip}, slice::Iter};

use crate::lexer::{TokenType, Token};

pub fn post_process(tokens: &mut Vec<Token>) {

    change_python(tokens);
    tokens.retain(|x: &Token| match x.token {
        TokenType::Whitespace(_) => false,
        _ => true,
    });

    scan_curlies(tokens);
    scan_sets(tokens);
    scan_classify_parens(tokens);
}

fn change_python(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len() {
        match &tokens[i].token {

            TokenType::Symbol(s) if s == ":" && tokens[i - 1].token == TokenType::GroupOpen(String::from("(")) => {
                let start: usize = advance_to(i, tokens, TokenType::LineFeed, -1);
                let num_tabs: usize;

                if start != tokens.len() {
                    println!("{}, {:?}", start, tokens[start]);
                    let mut j: usize = start + 1;
                    while j < tokens.len() && tokens[j].token == TokenType::Whitespace(String::from("\t")) {
                        j += 1;
                    }
                    num_tabs = (j - start) - 1;
                } else {
                    num_tabs = 0;
                }
                println!("{}", num_tabs);
                check_fun_tabs(num_tabs, tokens, i);
            }
            _ => (),
        }
    }
}

fn scan_curlies(tokens: &mut Vec<Token>) {
    for i in 1..tokens.len() {
        match &tokens[i].token {
            TokenType::GroupOpen(g) if g == "{" && tokens[i - 1].token != TokenType::Symbol(String::from("->")) => {
                tokens.insert(i, Token::new(TokenType::Symbol(String::from("->")), 2));
            }
            _ => (),
        }
    }
}

fn scan_sets(tokens: &mut Vec<Token>) {
    let mut to_add: Vec<usize> = Vec::new();
    for i in 0..tokens.len() {
        match &tokens[i].token {
            TokenType::Symbol(s) if s == "->" => {
                let index: usize = set_func_add(tokens, i);
                if index != tokens.len() {
                    to_add.push(index);
                }
            }
            _ => (),
        }
    }
    let mut add_sad: usize = 0;
    for index in to_add {
        if index != usize::MAX {
            tokens.insert(index + add_sad, Token::new(TokenType::Symbol(String::from("=")), 1));
            add_sad += 1;
        }
    }
}

fn scan_classify_parens(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len() {
        match &tokens[i].token {
            TokenType::GroupOpen(g) if g == "(" => paren_classify(tokens, i),
            _ => (),
        }
    }
}

fn set_func_add(tokens: &Vec<Token>, i: usize) -> usize {
    let j: usize = advance_to(i - 1, tokens, TokenType::GroupOpen(String::from("(")), -1);
    return match tokens[j - 1].token {
        TokenType::Identifier(_) => j,
        _ => usize::MAX,
    };
}

fn paren_classify(tokens: &mut Vec<Token>, i: usize) {
    let mut count: i32 = 1;
    let mut j = i;
    while count > 0 {
        match &tokens[j + 1].token {
            TokenType::GroupOpen(g) if g == "(" => count += 1,
            TokenType::GroupClose(g) if g == ")" => count -= 1,
            _ => (),
        }
        j += 1;
    }

    match tokens[i - 1].token {
        TokenType::Identifier(_) => {
            // tokens[i] = TokenType::LeftCallParenthesis;
            // tokens[j] = TokenType::RightCallParenthesis;
            return;
        }
        _ => (),
    }

    let to_add: (TokenType, TokenType);
    // if tokens[j + 1] == TokenType::Symbol(String::from("->")) {
    //     to_add = (
    //         TokenType::LeftFuncParenthesis,
    //         TokenType::RightFuncParenthesis,
    //     );
    // } else {
    //     to_add = (
    //         TokenType::LeftCaptureParenthesis,
    //         TokenType::RightCaptureParenthesis,
    //     );
    // }

    // tokens[i] = to_add.0;
    // tokens[j] = to_add.1;
}

fn check_fun_tabs(num_tabs: usize, tokens: &mut Vec<Token>, start_dex: usize) {
    let mut cur_tabs: usize;
    let mut token_iter: Skip<Enumerate<Iter<Token>>> = tokens.iter().enumerate().skip(start_dex);
    let mut index: usize = 0;
    
    while let Some((i, t)) = token_iter.next() {
        index = i;
        match &t.token {
            TokenType::LineFeed if i < tokens.len() - 1 => {
                let mut tabs: (usize, &Token) = token_iter.next().unwrap();
                cur_tabs = 0;
                while tabs.0 < tokens.len() && tabs.1.token == TokenType::Whitespace(String::from("\t")) {
                    cur_tabs += 1;  
                    tabs = token_iter.next().unwrap();
                }
                if cur_tabs <= num_tabs {
                    break;
                }
            }

            _ => (),
        }
    }
    tokens[start_dex].token = TokenType::GroupOpen(String::from("{"));

    let dex: usize = advance_to(index, tokens, TokenType::LineFeed, -1) + 1;
    tokens.insert(dex, Token::new(TokenType::GroupClose(String::from("}")), 1));
    tokens.insert(dex + 1, Token::new(TokenType::LineFeed, 1));
}

fn advance_to(start: usize, array: &Vec<Token>, token: TokenType, sign: i32) -> usize {
    let mut i: usize = start;
    while array[i].token != token {
        i = ((i as i32) + sign) as usize;
        if (sign < 0 && i > array.len()) || (sign > 0 && i == 0) {
            return array.len();
        }
    }
    return i;
}