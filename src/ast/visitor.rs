use super::ast_node::AstNode;

pub trait Visitor {
    type Output;

    fn visit<T: AstNode>(&mut self, node: T) -> Self::Output;
}


pub fn visit<A: AstNode, V: Visitor>(node: A, visitor: V) -> V::Output {
    visitor.visit(node)
}