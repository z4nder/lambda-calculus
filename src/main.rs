use std::fmt;

struct Var {
    name: String,
}

struct App {
    left: Box<Expr>,
    right: Box<Expr>,
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

fn main() {
    println!("{}", lambda_if());
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