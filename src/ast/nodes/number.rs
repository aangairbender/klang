use crate::ast::ast_node::AstNode;

pub struct NumberNode(pub i32);

impl AstNode for NumberNode {
    type Output = i32;
    fn eval(&self) -> i32 { self.0 }
}
