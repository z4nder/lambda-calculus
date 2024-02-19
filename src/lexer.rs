/**
 * 
 * λx.((x) (x))
 [x] Create tokens
 [X] Recive text input
 [X] Explode chars
 [X] Convert chars to array of tokens 
*/

#[derive(Debug)]
pub enum Token {
    Lambda,
    LParen,
    RParen,
    Dot,
    Var(String),
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
            token if token.is_ascii_alphabetic() => tokens.push(Token::Var(ch.to_string())),
            _ => {
                panic!("unrecognized char");
            }
        }
    }

    tokens
}