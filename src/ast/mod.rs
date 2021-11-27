pub enum Stmt {
    Let(String, Expr),
    Expr(Expr),
}

pub enum Expr {
    Factor(Factor),
    Add(Box<Factor>, Box<Expr>),
    Sub(Box<Factor>, Box<Expr>),
}

pub enum Factor {
    Unary(Unary),
    Multiply(Box<Unary>, Box<Factor>),
    Divide(Box<Unary>, Box<Factor>),
}

pub enum Unary {
    Minus(Box<Unary>),
    Primary(Primary),
}

pub enum Primary {
    Literal(i32),
    Identifier(String),
    Grouping(Box<Expr>)
}
