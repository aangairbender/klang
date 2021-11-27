use crate::ast::ast_node::AstNode;

pub struct IdentifierNode(pub String);

impl AstNode for IdentifierNode {
    type Output = i32;
    fn eval(&self) -> i32 { self.0 }
}
