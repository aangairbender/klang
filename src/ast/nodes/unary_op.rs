use crate::ast::ast_node::AstNode;


pub enum UnaryOp {
    Minus
}

pub struct UnaryOpNode {
    op: UnaryOp,
    rhs: Box<dyn AstNode<Output = i32>>
}

impl UnaryOpNode {
    pub fn new(op: UnaryOp, rhs: Box<dyn AstNode<Output = i32>>) -> Self {
        Self { op, rhs }
    }
}

impl AstNode for UnaryOpNode {
    type Output = i32;
    fn eval(&self) -> i32 {
        match self.op {
            UnaryOp::Minus => -self.rhs.eval(),
        }
    }
}
