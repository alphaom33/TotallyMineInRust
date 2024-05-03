use crate::TokenTypes;

pub fn postProcess(tokens: &Vec<TokenTypes>) {
    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenTypes::Name(c) => parenAdd(tokens, i),
            _ => continue,
        }
    }
}

fn parenAdd(tokens: &Vec<TokenTypes>, i: usize) {
    if tokens[i + 1] != TokenTypes::LeftParenthesis {
        return;
    }
    let mut j: usize = i + 2;
    while tokens[j] != TokenTypes::RightParenthesis {
        j += 1;
    }

    let mut k = j;
    while tokens[k] == TokenTypes::LineFeed {
        k += 1;
        if k > tokens.len() {
            return;
        }
    }

    
}
