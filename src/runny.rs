use std::{
    borrow::Borrow,
    collections::HashMap,
    iter::{Map, Peekable},
    mem::transmute,
    os::raw::c_void,
    ptr::{self, null, null_mut},
    rc::Rc,
    slice::Iter,
};

use once_cell::sync::Lazy;

use crate::types::BaseValue;
use crate::{
    lexer::{Token, TokenType},
    types::Num,
};

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
                        while token_iter.peek().is_some() && *token_iter.peek().unwrap() != &TokenType::LineFeed {
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
    println!("{:?}", get_bases);
    if get_bases.iter().any(|x: &TokenType| {
        if let TokenType::Value(y) = x {
            match y {
                BaseValue::String(_) => true,
                _ => false,
            }
        } else {
            false
        }
    }) {
        let mut out: String = String::new();
        for x in get_bases {
            if let TokenType::Value(v) = x {
                out += &v.to_string();
            };
        }
        BaseValue::String(out)
    } else if get_bases.iter().any(|x: &TokenType| {
        if let TokenType::Value(v) = x {
            match v {
                BaseValue::Number(_) => true,
                _ => false,
            }
        } else {
            false
        }
    }) {
        numbebrefdss(get_bases)
    } else {
        BaseValue::Null
    }
}

unsafe fn numbebrefdss(thingy: Vec<TokenType>) -> BaseValue {
    let mut thingy_iter: Peekable<Iter<TokenType>> = thingy.iter().peekable();
    let mut sum: f64 = if let TokenType::Value(BaseValue::Number(n)) = thingy_iter.next().unwrap() {
        num_bool(n)
    } else {
        0.0f64
    };

    while let Some(t) = thingy_iter.next() {
        match t {
            TokenType::Symbol(s) if s == "+" => {
                if let TokenType::Value(BaseValue::Number(Num::Float(f))) = thingy_iter.next().unwrap() {
                    sum += f;
                }
            },
            _ => panic!()
        }
    }

    BaseValue::Number(Num::Float(sum))
}

fn num_bool(num: &Num) -> f64 {
    match num {
        Num::Float(f) => *f,
        Num::Boolean(b) => if *b {1.0f64} else {0.0f64}, 
        _ => panic!(),
    }
}

unsafe fn typeify(expression: &mut Vec<&TokenType>) -> Vec<TokenType> {
    let mut i: usize = 0;
    let mut get_bases: Vec<TokenType> = Vec::new();
    while i < expression.len() {
        println!("{:?}", expression[i]);
        get_bases.push(match &expression[i] {
            TokenType::Identifier(idy) => {
                if expression[i + 1] == &TokenType::GroupOpen(String::from("(")) {
                    call_function(expression, &mut i)
                } else {
                    TokenType::Value(VARS.get(idy).unwrap().clone())
                }
            },
            TokenType::Number(n) => TokenType::Value(BaseValue::Number(Num::Float(n.parse::<f64>().unwrap()))),
            TokenType::Symbol(s) if ["+"].contains(&s.as_str()) => TokenType::Symbol(s.to_string()),
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
