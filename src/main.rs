mod lexer;
mod parser;
mod eval;

use lexer::lexer;
use parser::parser;
use eval::evaluate;


fn main() {
    // λx.((x) (x))
    // https://professor.pucgoias.edu.br/SiteDocente/admin/arquivosUpload/4442/material/Trabalho_ED1_-_Pilha_e_Fila%20-%202013-2.pdf
    let input: String = String::from("λx.λy. x"); 
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

    // Question input values
}