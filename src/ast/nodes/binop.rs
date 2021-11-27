use crate::ast::ast_node::AstNode;

pub enum Associativity {
    Left,
    Right
}

pub struct BinOpData {
    associativity: Associativity,
    precedence: u8
}

pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide
}

impl BinOp {
    pub fn data(&self) -> BinOpData {
        match &self {
            BinOp::Plus => BinOpData { associativity: Associativity::Left, precedence: 1 },
            BinOp::Minus => BinOpData { associativity: Associativity::Left, precedence: 1 },
            BinOp::Multiply => BinOpData { associativity: Associativity::Left, precedence: 2 },
            BinOp::Divide => BinOpData { associativity: Associativity::Left, precedence: 2 },
        }
    }
}

pub struct BinOpNode {
    op: BinOp,
    lhs: Box<dyn AstNode<Output = i32>>,
    rhs: Box<dyn AstNode<Output = i32>>
}

impl BinOpNode {
    pub fn new(op: BinOp, lhs: Box<dyn AstNode<Output = i32>>, rhs: Box<dyn AstNode<Output = i32>>) -> Self {
        Self { op, lhs, rhs }
    }
}

impl AstNode for BinOpNode {
    type Output = i32;
    fn eval(&self) -> i32 {
        match self.op {
            BinOp::Plus => self.lhs.eval() + self.rhs.eval(),
            BinOp::Minus => self.lhs.eval() - self.rhs.eval(),
            BinOp::Multiply => self.lhs.eval() * self.rhs.eval(),
            BinOp::Divide => self.lhs.eval() / self.rhs.eval()
        }
    }
}
