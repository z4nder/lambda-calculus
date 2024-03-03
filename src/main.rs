mod lexer;
mod parser;
mod eval;

use lexer::lexer;
use parser::parser;
use eval::evaluate;


fn main() {
    // Inputs:
    // λx.((x) (x))
    // λx.λy. y
    // λx.+ x 1 notação polonesa 
    
    let input: String = String::from("λx.+ x 1"); 
    let tokens = lexer(input);

    let result = parser(tokens);

    match result {
        Ok(expr) => {
            match evaluate(expr) {
                Ok(value) => println!("Result: {}", value),
                Err(err) => println!("Errro: {}", err),       
            }
        },
        Err(err) => println!("Errro: {}", err),        
    };
}