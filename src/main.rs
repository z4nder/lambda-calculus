mod lexer;
mod parser;

use lexer::lexer;
use parser::parser;


fn main() {
    // λx.((x) (x))
    let input = String::from("λx.λy. y"); 
    let tokens = lexer(input);

    let result = parser(tokens);

    match result {
        Ok(expr) => println!("Result: {}", expr),
        Err(err) => println!("ERROR => {}", err),        
    }
}