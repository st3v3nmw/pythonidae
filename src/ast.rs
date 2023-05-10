#[derive(Debug)]
pub enum Stmt {
    RawExpr(Box<Expr>),
    Assignment(String, Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Op(Box<Expr>, OpCode, Box<Expr>),
}

#[derive(Debug)]
pub enum OpCode {
    Mul,
    Div,
    Add,
    Sub,
}
