/*
 λx.((x) (x))
 [] Recive tokens
 [] Replace tokens to values
 [] Output AST
*/

use std::fmt;

use crate::lexer::Token;

pub struct Var {
    name: String,
}

pub struct App { 
    left: Box<Expr>, 
    right: Box<Expr>,
}

pub struct Lam {
    param: String,
    body: Option<Box<Expr>>,
}

pub enum Expr {
    Variable(Var),
    Application(App),
    Lambda(Lam),
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) ({})", self.left, self.right)
    }
}

impl fmt::Display for Lam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "λ{}.({})", self.param, self.body)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Variable(var) => write!(f, "{}", var),
            Expr::Application(app) => write!(f, "{}", app),
            Expr::Lambda(lam) => write!(f, "{}", lam),
        }
    }
}


pub fn parser(tokens: Vec<Token>)  {
    let mut tokens: Vec<Token> = Vec::new();

    let mut ast: Vec<Expr> = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Lambda => {
                let expression = parse_lambda(&tokens, index);
            }
            _ => panic!("Failed to parse")
        }
    }
}

pub fn parse_lambda(tokens: &Vec<Token>, index: usize) -> Result<Expr, String>{
    match tokens.get(index+1) {
        Some(next_token) => {
            match next_token {
                Token::Var(param_name) => {
                    match parse_dot(tokens, index) {
                        None => Ok(Expr::Lambda(Lam {
                            param: param_name.clone(),
                            body: None,
                            })),
                        Some(err) => Err(err)    
                    }
                   
                }
                _ => Err("Invalid char {TOKEN}".to_string())
            }
        }
        None => Err("Incompleted lambda expression".to_string())
    }    
}

pub fn parse_dot(tokens: &Vec<Token>, index: usize) -> Option<String>{
    match tokens.get(index+1) {
        Some(next_token) => {
            match next_token {
                Token::Dot => {
                   None                   
                }
                _ => Some("Invalid char {TOKEN}, expected '.'".to_string())
            }
        }
        None => Some("Incompleted lambda expression, expected '.'".to_string())
    }    
}
// λx.((x) (x))