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
        match &self.body {
            Some(body) => write!(f, "λ{}.({})", self.param, body),
            None => write!(f, "Empty body λ expression"),
        }
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


pub fn parser(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut expression: Result<Expr, String> = Err("Empty expression".to_string());

    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Lambda => {
                expression = Ok(parse_lambda(&tokens, index+1)?);
            }
            Token::LParen => todo!(),
            Token::RParen => todo!(),
            Token::Dot => todo!(),
            Token::Var(_) => todo!(),
            _ => return Err(format!("Invalid char {:?}", token)),
        }
    }
    
    expression
}

pub fn parse_lambda(tokens: &Vec<Token>, next_index: usize) -> Result<Expr, String>{
    match tokens.get(next_index) {
        Some(next_token) => {
            match next_token {
                //  Call parse Here()
                Token::Var(param_name) => {
                    match parse_dot(tokens, next_index+1) {
                        None => Ok(Expr::Lambda(Lam {
                            param: param_name.clone(),
                            body: None,
                        })),
                        Some(err) => Err(err)    
                    }
                   
                }
                _ => Err("Invalid char {TOKEN}, expected 'Var'".to_string())
            }
        }
        None => Err("Incompleted lambda expression, expected 'Var'".to_string())
    }    
}

pub fn parse_dot(tokens: &Vec<Token>, next_index: usize) -> Option<String>{
    match tokens.get(next_index) {
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