use std::io;

use crate::parser::{App, Expr, Int, Lam};

pub fn evaluate(expr: Expr) -> Result<Expr, String> {
    match expr {
        Expr::Lambda(lam) => {
            let param_name = lam.param;
            let param_value = get_param(param_name.clone());

            let binded_body = bind_expressions(param_value, param_name, *lam.body)?;

            Ok(evaluate(binded_body)?)
        },
        Expr::Application( app) => {    
            let left = app.left;        
            let right = app.right;     

            match *left {
                Expr::Variable(var) if var.name == "+".to_string() => eval_plus(*right),
                _ => {
                    let evaluated_left = evaluate(*left)?;
                    let evaluated_right = evaluate(*right)?;

                    println!("Evaluated Left: {}", evaluated_left);
                    println!("Evaluated Right: {}", evaluated_right);

                    Ok(Expr::Application(App {
                        left: Box::new(evaluated_left),
                        right: Box::new(evaluated_right),
                    }))
                }
            }
        }
        _ => Ok(expr),
    }
}

pub fn eval_plus(expr:Expr) -> Result<Expr, String>{
    match expr {
        Expr::Application(app) => {
            match (*app.left, *app.right) {
                (Expr::VariableInt(left), Expr::VariableInt(right)) => Ok(Expr::VariableInt(Int { value: left.value + right.value })),
                _ => Err("Invalid Sum values".to_string()),
            }
        }
        _ => Err("Invalid Sum expression".to_string()),
    }
}

pub fn bind_expressions(param: i32, original: String, body: Expr) -> Result<Expr, String> {
    match body {
        Expr::Variable(var) if var.name == original => Ok(Expr::VariableInt(Int { value: param })),
        Expr::Application(app) => {
            let left = bind_expressions(param, original.clone(), *app.left)?;
            let right = bind_expressions(param, original.clone(), *app.right)?;

            Ok( Expr::Application(App {
                left: Box::new(left),
                right: Box::new(right),
            }))
        }
        Expr::Lambda(lam) => {
            let param_name = lam.param;
            let binded_body = bind_expressions(param, original.clone(), *lam.body)?;

            Ok(Expr::Lambda(Lam {
                param: param_name,
                body: Box::new(binded_body),
            }))
        }
        _ => Ok(body)
    }
}

pub fn get_param(string: String) -> i32 {
    println!("Qual o valor de: '{}' ?", string);

    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).unwrap();

    input_string.trim().parse().unwrap()
}
