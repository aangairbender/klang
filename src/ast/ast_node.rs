use super::visitor::Visitor;

pub trait AstNode {
    type Output;

    fn eval(&self) -> Self::Output;
}