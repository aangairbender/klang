pub struct Name {
    value: String
}

pub enum Stmt {
    Expr(Expr),
    Let(Name, Expr)
}

pub enum Expr {
    Factor(Factor),
    Add(Factor, Factor),
    Sub(Factor, Factor),
}

pub enum Factor {
    Unary(Unary),
    Multiply(Box<Unary>, Box<Unary>),
    Divide(Box<Unary>, Box<Unary>),
}

pub enum Unary {
    Primary(Primary),
    Minus(Box<Unary>),
}

pub enum Primary {
    Literal,
    Identifier(Name),
}
