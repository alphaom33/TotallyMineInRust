use std::{collections::HashMap, iter::{Enumerate, Peekable}, slice::Iter};

use once_cell::sync::Lazy;

use crate::{lexer::TokenType, types::BaseValue, types::Num};

static mut VARS: Lazy<HashMap<String, BaseValue>> = Lazy::new(HashMap::new);

pub unsafe fn run(tokens: Vec<TokenType>) {
    let mut token_iter: Peekable<Iter<TokenType>> = tokens.iter().peekable();
    while let Some(c) = token_iter.next() {
        match c {
            TokenType::Identifier(i) => match i {
                _ => {
                    if *token_iter.peek().unwrap() == &TokenType::Symbol(String::from("=")) {
                        token_iter.next();
                        let mut to_evaluate: Vec<&TokenType> = Vec::new();
                        while token_iter.peek().is_some()
                            && *token_iter.peek().unwrap() != &TokenType::LineFeed
                        {
                            to_evaluate.push(token_iter.next().unwrap());
                        }
                        VARS.insert(i.to_string(), eval_expression(&mut to_evaluate));
                    } else {
                        //call function?
                    }
                }
            },
            _ => (),
        }
    }
    println!("{:?}", VARS);
}

unsafe fn eval_expression(expression: &mut Vec<&TokenType>) -> BaseValue {
    let get_bases: Vec<TokenType> = typeify(expression);
    if contains(&get_bases, BaseValue::String(String::new())) {
        eval_strings(get_bases)
    } else if contains(&get_bases, BaseValue::Number(Num::Float(0.0f64))) {
        eval_numbers(get_bases)
    } else {
        panic!()
    }
}

fn contains(expression: &Vec<TokenType>, value_type: BaseValue) -> bool {
    expression.iter().any(|x| {
        if let TokenType::Value(v) = x {
            match v {
                BaseValue::Number(_) if value_type == BaseValue::Number(Num::Float(0.0f64)) => true,
                BaseValue::String(_) if value_type == BaseValue::String(String::new()) => true,
                _ => false,
            }
        } else {
            false
        }
    })
}

unsafe fn eval_strings(thingy: Vec<TokenType>) -> BaseValue {
    let mut out: String = String::new();
    for x in thingy {
        if let TokenType::Value(BaseValue::String(s)) = x {
            out += &s;
        };
    }
    BaseValue::String(out)
}

unsafe fn eval_numbers(thingy: Vec<TokenType>) -> BaseValue {
    let mut thingy_iter: Peekable<Enumerate<Iter<TokenType>>> = thingy.iter().enumerate().peekable();
    let mut sum: f64 = if let TokenType::Value(BaseValue::Number(n)) = thingy_iter.next().unwrap().1 {
        num_bool(n)
    } else {
        0.0f64
    };

    while let Some(t) = thingy_iter.next() {
        match t.1 {
            TokenType::Symbol(s) => {
                if let TokenType::Value(BaseValue::Number(Num::Float(f))) =
                    thingy_iter.next().unwrap().1
                {
                    match_operands(&mut sum, s, *f);
                } else {
                    let start: usize = thingy_iter.peek().unwrap().0;
                    while *thingy_iter.peek().unwrap().1 != TokenType::GroupClose(String::from(")")) {
                        thingy_iter.next();
                    }
                    let vecy = thingy[start..thingy_iter.peek().unwrap().0].to_vec();
                    println!("{:?}", vecy);
                    if let BaseValue::Number(Num::Float(f)) = eval_numbers(vecy) {
                        match_operands(&mut sum, s, f);
                    }
                    thingy_iter.next();
                }
            }
            _ => {
                println!("{:?}, {:?}", thingy, t.1);
                panic!()
            },
        }
    }

    BaseValue::Number(Num::Float(sum))
}

fn match_operands(sum: &mut f64, s: &String, f: f64) {
    match s.as_str() {
        "+" => *sum += f,
        "-" => *sum -= f,
        "*" => *sum *= f,
        "/" => *sum /= f,
        _ => (),
    }
}

fn num_bool(num: &Num) -> f64 {
    match num {
        Num::Float(f) => *f,
        Num::Boolean(b) => {
            if *b {
                1.0f64
            } else {
                0.0f64
            }
        }
        _ => panic!(),
    }
}

unsafe fn typeify(expression: &mut Vec<&TokenType>) -> Vec<TokenType> {
    let mut i: usize = 0;
    let mut get_bases: Vec<TokenType> = Vec::new();
    while i < expression.len() {
        get_bases.push(match &expression[i] {
            TokenType::Identifier(idy) => {
                if expression[i + 1] == &TokenType::GroupOpen(String::from("(")) {
                    call_function(expression, &mut i)
                } else {
                    TokenType::Value(VARS.get(idy).unwrap().clone())
                }
            }
            TokenType::Number(n) => {
                TokenType::Value(BaseValue::Number(Num::Float(n.parse::<f64>().unwrap())))
            }
            TokenType::Symbol(s) if "+-/*".contains(&s.as_str()) => {
                TokenType::Symbol(s.to_string())
            }
            TokenType::GroupOpen(s) if "(" == s.as_str() => TokenType::GroupOpen(String::from("(")),
            TokenType::GroupClose(s) if ")" == s.as_str() => TokenType::GroupClose(String::from(")")),
            TokenType::String(s) => TokenType::Value(BaseValue::String(s.to_string())),
            _ => TokenType::Comment,
        });

        i += 1;
    }
    get_bases
}

fn call_function(expression: &mut Vec<&TokenType>, i: &mut usize) -> TokenType {
    while expression[*i] != &TokenType::GroupClose(String::from(")")) {
        *i += 1;
    }
    *i += 1;
    TokenType::Value(BaseValue::Null)
}
