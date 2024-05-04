use crate::TokenType;

pub fn post_process(tokens: &mut Vec<TokenType>) {
    if tokens.last().unwrap() != &TokenType::LineFeed {
        tokens.push(TokenType::LineFeed);
    }

    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenType::Name(_) => paren_add(tokens, i),
            _ => (),
        }
    }
    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenType::Lambda if tokens[i + 1] != TokenType::LeftCurlyBracket => {
                curly_add(tokens, i)
            }
            _ => (),
        }
    }
    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenType::LeftParenthesis => paren_classify(tokens, i),
            _ => (),
        }
    }
}

fn paren_add(tokens: &mut Vec<TokenType>, i: usize) {
    if tokens[i + 1] != TokenType::LeftParenthesis {
        return;
    }

    let mut j: usize = i + 2;
    while tokens[j] != TokenType::RightParenthesis {
        j += 1;
        if j > tokens.len() {
            return;
        }
    }

    let mut k = j + 1;
    while tokens[k] == TokenType::LineFeed {
        k += 1;
        if k > tokens.len() {
            return;
        }
    }
    if tokens[k] != TokenType::LeftCurlyBracket {
        return;
    }

    tokens.insert(i + 1, TokenType::Set);
    tokens.insert(k + 1, TokenType::Lambda);
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

    let mut k: usize = j + 1;
    while tokens[k] == TokenType::LineFeed {
        k += 1;
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
    while j < tokens.len() && ![TokenType::Comma].contains(&tokens[j]) && count > 0 {
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
