struct Interpreter;
impl Visitor<i32> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i32 { panic!() }
    fn visit_stmt(&mut self, s: &Stmt) -> i32 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i32 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }

    
    fn visit_stmt(&mut self, s: &Stmt) -> i32;
    fn visit_name(&mut self, n: &Name) -> i32;
    fn visit_expr(&mut self, e: &Expr) -> i32;
    fn visit_factor(&mut self, e: &Factor) -> i32;
    fn visit_unary(&mut self, e: &Unary) -> i32;
    fn visit_primary(&mut self, e: &Primary) -> i32 {
        match *e {
            Primary::Literal(literal) => ,
            Primary::Identifier(name) => ,
        }
    }
}