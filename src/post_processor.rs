use crate::TokenType;

pub fn post_process(tokens: &mut Vec<TokenType>) {
    if tokens.last().unwrap() != &TokenType::LineFeed {
        tokens.push(TokenType::LineFeed);
    }

    scan_curlies(tokens);
    scan_sets(tokens);
    scan_lambda_curlies(tokens);
    scan_classify_parens(tokens);
}

fn scan_curlies(tokens: &mut Vec<TokenType>) {
    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenType::LeftCurlyBracket if tokens[i - 1] != TokenType::Lambda => {
                tokens.insert(i, TokenType::Lambda)
            }
            _ => (),
        }
    }
}

fn scan_sets(tokens: &mut Vec<TokenType>) {
    let mut to_add: Vec<usize> = Vec::new();
    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenType::Lambda => {
                let index: usize = set_func_add(tokens, i);
                if  index != tokens.len() {
                    to_add.push(index);
                }
            },
            _ => (),
        }
    }
    let mut add_sad: usize = 0;
    for index in to_add {
        if index != usize::MAX {
            tokens.insert(index + add_sad, TokenType::Set);
            add_sad += 1;
        }
    }
}

fn scan_lambda_curlies(tokens: &mut Vec<TokenType>) {
    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenType::Lambda if tokens[i + 1] != TokenType::LeftCurlyBracket => {
                curly_add(tokens, i)
            }
            _ => (),
        }
    }
}

fn scan_classify_parens(tokens: &mut Vec<TokenType>) {
    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenType::LeftParenthesis => paren_classify(tokens, i),
            _ => (),
        }
    }    
}

fn set_func_add(tokens: &Vec<TokenType>, i: usize) -> usize {
    let j: usize = advance_to(i - 1, tokens, TokenType::LeftParenthesis, -1);
    return match tokens[j - 1] {
        TokenType::Name(_) => j,
        _ => usize::MAX,
    }
}

fn paren_classify(tokens: &mut Vec<TokenType>, i: usize) {
    let mut count: i32 = 1;
    let mut j = i;
    while count > 0 {
        match tokens[j + 1] {
            TokenType::LeftParenthesis => count += 1,
            TokenType::RightParenthesis => count -= 1,
            _ => (),
        }
        j += 1;
    }

    match tokens[i - 1] {
        TokenType::Name(_) => {
            tokens[i] = TokenType::LeftCallParenthesis;
            tokens[j] = TokenType::RightCallParenthesis;
            return;
        }
        _ => (),
    }

    let to_add: (TokenType, TokenType);
    if tokens[j + 1] == TokenType::Lambda {
        to_add = (
            TokenType::LeftFuncParenthesis,
            TokenType::RightFuncParenthesis,
        );
    } else {
        to_add = (
            TokenType::LeftCaptureParenthesis,
            TokenType::RightCaptureParenthesis,
        );
    }

    tokens[i] = to_add.0;
    tokens[j] = to_add.1;
}

fn curly_add(tokens: &mut Vec<TokenType>, i: usize) {
    let mut j: usize = i + 1;
    let mut count: i32 = 1;
    while j < tokens.len()
        && ![TokenType::Comma, TokenType::LineFeed].contains(&tokens[j])
        && count > 0
    {
        match tokens[j] {
            TokenType::LeftParenthesis => count += 1,
            TokenType::RightParenthesis => count -= 1,
            _ => (),
        }
        j += 1;
    }

    tokens.insert(i + 1, TokenType::LeftCurlyBracket);
    tokens.insert(j, TokenType::RightCurlyBracket);
}

fn advance_to(start: usize, array: &Vec<TokenType>, token: TokenType, sign: i32) -> usize {
    let mut i: usize = start;
    while array[i] != token {
        i = ((i as i32) + sign) as usize;
        if (sign < 0 && i > array.len()) || (sign > 0 && i == 0) {
            return array.len();
        }
    }
    return i;
}
