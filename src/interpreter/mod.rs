use std::collections::HashMap;

use crate::{ast::*, visit::Visitor};

pub struct Interpreter {
    variables: HashMap<String, i32>
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }
}

impl Visitor<i32> for Interpreter {    
    fn visit_stmt(&mut self, e: &Stmt) -> i32 {
        match e {
            Stmt::Let(a, b) => {
                let value = self.visit_expr(b);
                self.variables.insert(a.clone(), value);
                value
            },
            Stmt::Expr(a) => self.visit_expr(&a),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i32 {
        match e {
            Expr::Factor(f) => self.visit_factor(&f),
            Expr::Add(a, b) => self.visit_factor(&a) + self.visit_expr(&b),
            Expr::Sub(a, b) => self.visit_factor(&a) - self.visit_expr(&b),
        }
    }

    fn visit_factor(&mut self, e: &Factor) -> i32 {
        match e {
            Factor::Unary(u) => self.visit_unary(&u),
            Factor::Multiply(a, b) => self.visit_unary(a.as_ref()) * self.visit_factor(&b),
            Factor::Divide(a, b) => self.visit_unary(a.as_ref()) / self.visit_factor(&b),
        }
    }

    fn visit_unary(&mut self, e: &Unary) -> i32 {
        match e {
            Unary::Minus(u) => -self.visit_unary(u.as_ref()),
            Unary::Primary(p) => self.visit_primary(&p),
        }
    }

    fn visit_primary(&mut self, e: &Primary) -> i32 {
        match e {
            Primary::Literal(literal) => *literal,
            Primary::Identifier(name) => self.variables[name],
            Primary::Grouping(expr) => self.visit_expr(expr.as_ref()),
        }
    }
}