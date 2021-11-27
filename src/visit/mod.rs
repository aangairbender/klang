use crate::ast::*;

pub trait Visitor<T> {
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_name(&mut self, n: &Name) -> T;
    fn visit_expr(&mut self, e: &Expr) -> T;
    fn visit_factor(&mut self, e: &Factor) -> T;
    fn visit_unary(&mut self, e: &Unary) -> T;
    fn visit_primary(&mut self, e: &Primary) -> T;
}