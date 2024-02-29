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
    body: Box<Expr>,
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


pub fn parser(tokens:  Vec<Token>) -> Result<Expr, String> {    
    match tokens.get(0) {
        Some(token) => {
            match token {
                Token::LParen => return Ok(parse_paren(tokens)?),
                Token::Var(var) =>  return Ok(parse_var(tokens.clone(), var)),
                Token::Lambda =>  return Ok(parse_lambda(tokens)?),                
                Token::RParen =>  return parse_paren(tokens),
                Token::Dot => return Err("Invalid char '.'".to_string())   
            };
        }
        None => return Err("Empty expression".to_string())
    }  
}

pub fn parse_paren(tokens: Vec<Token>) -> Result<Expr, String>{
    // Remove #(
    let tokens = remove_token(tokens);

   parser(tokens)
}

pub fn parse_var(tokens: Vec<Token>, var: &String) -> Expr {  
    // Remove #Var
    let tokens = remove_token(tokens);

    match parser(tokens) {
        Ok(expr) => {
            return Expr::Application(App {
                left: Box::new(Expr::Variable(Var {
                    name: var.clone(),
                })),
                right: Box::new(expr),
            })
        },
        _ => Expr::Variable(Var {
               name: var.clone(),
             })
    }
   
}

pub fn parse_lambda(tokens: Vec<Token>) -> Result<Expr, String>{
    // Remove #λ
    let tokens = remove_token(tokens);
    let param_name = parser(tokens.clone())?;
    // Remove #Var
    let tokens = remove_token(tokens);

    match param_name {
        Expr::Variable(var) => {
            // Remove #.
            let tokens= remove_token(tokens);
            
            return Ok(Expr::Lambda(Lam {
                param: var.name,
                body: Box::new(parser(tokens)?),
            }));
        },
        _ =>  Err("Invalid char {TOKEN}, expected 'Var'".to_string())
    }
}

pub fn remove_token(mut tokens: Vec<Token>) -> Vec<Token>{
    (*tokens.drain(1..).collect::<Vec<Token>>()).to_vec()
}