use crate::expr::Expr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("{0} {1} {2}")]
    InvalidBinaryExpr(Expr, String, Expr),
    #[error("{0} {1}")]
    InvalidUnaryExpr(String, Expr),
    #[error("{0}")]
    Error(String),
}
