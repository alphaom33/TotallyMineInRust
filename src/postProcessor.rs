use crate::TokenTypes;

pub fn post_process(tokens: &mut Vec<TokenTypes>) {
    for i in 0..tokens.len() {
        match &tokens[i] {
            TokenTypes::Name(_) if i < tokens.len() - 3 => paren_add(tokens, i),
            _ => continue,
        }
    }
}

fn paren_add(tokens: &mut Vec<TokenTypes>, i: usize) {
    if tokens[i + 1] != TokenTypes::LeftParenthesis {
        return;
    }

    let mut j: usize = i + 2;
    while tokens[j] != TokenTypes::RightParenthesis {
        j += 1;
        if j > tokens.len() {
            return;
        }
    }
    println!("{}", tokens[j]);

    let mut k = j;
    while tokens[k] == TokenTypes::LineFeed {
        k += 1;
        if k > tokens.len() {
            return;
        }
    }

    tokens.insert(i + 1, TokenTypes::Set);
    tokens.insert(k + 2, TokenTypes::Lambda);
}
