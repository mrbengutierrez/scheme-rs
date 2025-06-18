#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    Boolean(bool),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
}