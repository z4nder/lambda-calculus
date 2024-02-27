use std::fmt;

mod lexer;
mod parser;

use lexer::lexer;
use parser::parser;

struct Var {
    name: String,
}

struct App { 
    left: Box<Expr>, // Function f
    right: Box<Expr>, // Expression x -> f(x)
}

struct Lam {
    param: String,
    body: Box<Expr>,
}

enum Expr {
    Variable(Var),
    Application(App),
    Lambda(Lam),
}



fn main() {
    let input = String::from("λx.((x) (x))");
    let tokens = lexer(input);

    let result = parser(tokens);

    match result {
        Ok(expr) => println!("{}", expr),
        Err(err) => println!("ERROR => {}", err),        
    }
}

fn lambda_expression_test() -> Expr {
    // λx.x x = λx.((x) (x))
    // JS (x => x(x))
    Expr::Lambda(Lam {
        param: "x".to_string(),
        body: Box::new(Expr::Application(App {
            left: Box::new(Expr::Variable(Var {
                name: "x".to_string(),
            })),
            right: Box::new(Expr::Variable(Var {
                name: "x".to_string(),
            })),
        })),
    })
}

fn lambda_true() -> Expr {
    // λx.λy.x
    // JS (x => y => x)
    Expr::Lambda(Lam {
        param: "TRUE".to_string(),
        body: Box::new(Expr::Lambda(Lam {
            param: "FALSE".to_string(),
            body: Box::new(Expr::Variable(Var {
                name: "TRUE".to_string(),
            })),
        })),
    })
}

fn lambda_false() -> Expr {
    // λx.λy.x
    // JS (x => y => y)
    Expr::Lambda(Lam {
        param: "TRUE".to_string(),
        body: Box::new(Expr::Lambda(Lam {
            param: "FALSE".to_string(),
            body: Box::new(Expr::Variable(Var {
                name: "FALSE".to_string(),
            })),
        })),
    })
}

fn lambda_if() -> Expr {
    // λx.λy.λz.x y z
    // JS (x => y => z => x(y(z)))
    Expr::Lambda(Lam {
        param: "IF".to_string(),
        body: Box::new(Expr::Lambda(Lam {
            param: "TRUE".to_string(),
            body: Box::new(Expr::Lambda(Lam {
                param: "FALSE".to_string(),
                body: Box::new(Expr::Application(App {
                    left: Box::new(Expr::Variable(Var{ name: "IF".to_string()})),
                    right:  Box::new(Expr::Application(App {
                        left: Box::new(Expr::Variable(Var{ name: "TRUE".to_string()})),
                        right:  Box::new(Expr::Variable(Var{ name: "FALSE".to_string()}))
                    }))
                })),
            })),
        })),
    })
}

// Lexer
// Parser