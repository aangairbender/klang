use super::{nodes::NumberNode, visitor::Visitor};

pub struct EvalVisitor;

impl Visitor for EvalVisitor {
    type Output = i32;

    fn visit<T: super::ast_node::AstNode>(&mut self, node: T) -> Self::Output {
    }
}