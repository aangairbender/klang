use crate::ast::ast_node::AstNode;

pub struct ExprNode;

impl AstNode for ExprNode {
    type Output = i32;

    fn eval(&self) -> Self::Output {
        todo!()
    }
}