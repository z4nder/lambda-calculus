#[derive(Debug, Clone)]
pub enum Token {
    Lambda,
    LParen,
    RParen,
    Dot,
    Var(String),
    Int(i32),
    Plus,
    Minus
}



pub fn lexer(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            'λ' => tokens.push(Token::Lambda),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '.' => tokens.push(Token::Dot),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            token if token.is_numeric() => tokens.push(Token::Int(ch.to_digit(10).unwrap() as i32)),
            token if token.is_ascii_alphabetic() => tokens.push(Token::Var(ch.to_string())),
            _ => {
                panic!("unrecognized char");
            }
        }
    }

    tokens
}