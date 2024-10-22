use crate::{expr::Expr, stmt::Stmt, value::Value};

pub struct Optimizer;

impl Optimizer {
    pub fn optimize(stmts: &[Stmt]) -> Vec<Stmt> {
        stmts
            .iter()
            .cloned()
            .map(|s| Self::optimize_stmt(&s))
            .collect()
    }

    fn optimize_stmt(stmt: &Stmt) -> Stmt {
        match stmt {
            Stmt::Exit(_) | Stmt::Block(_) | Stmt::Func(..) | Stmt::Return(..) => stmt.clone(),
            Stmt::Print(expr) => Stmt::Print(Self::optimize_expr(expr)),
            Stmt::Expr(expr) => Stmt::Expr(Self::optimize_expr(expr)),
            Stmt::If(cond, body) => match Self::optimize_expr(cond) {
                Expr::Literal(Value::Bool(true)) => Stmt::Block(body.to_vec()),
                Expr::Literal(Value::Bool(false)) => Stmt::Expr(Expr::Literal(Value::Null)),
                _ => stmt.clone(),
            },
            Stmt::Assign(s, expr) => Stmt::Assign(s.to_string(), Self::optimize_expr(expr)),
        }
    }

    fn optimize_expr(expr: &Expr) -> Expr {
        match expr {
            Expr::Literal(value) => match value {
                Value::Bool(_) | Value::Num(_) | Value::String(_) | Value::Null => expr.clone(),
                Value::Array(vec) => {
                    let mut items = vec![];
                    for item in vec {
                        items.push(Self::optimize_expr(item));
                    }
                    Expr::Literal(Value::Array(items))
                }
            },
            Expr::Add(l, r) => {
                let (l, r) = (Self::optimize_expr(l), Self::optimize_expr(r));
                match (l, r) {
                    (Expr::Literal(Value::Num(x)), Expr::Literal(Value::Num(y))) => {
                        Expr::Literal(Value::Num(x + y))
                    }
                    (Expr::Literal(Value::String(mut x)), Expr::Literal(Value::String(y))) => {
                        x.push_str(&y);
                        Expr::Literal(Value::String(x))
                    }
                    _ => expr.clone(),
                }
            }
            Expr::Sub(l, r) => {
                let (l, r) = (Self::optimize_expr(l), Self::optimize_expr(r));
                match (l, r) {
                    (Expr::Literal(Value::Num(x)), Expr::Literal(Value::Num(y))) => {
                        Expr::Literal(Value::Num(x - y))
                    }
                    _ => expr.clone(),
                }
            }
            Expr::Mul(l, r) => {
                let (l, r) = (Self::optimize_expr(l), Self::optimize_expr(r));
                match (l, r) {
                    (Expr::Literal(Value::Num(x)), Expr::Literal(Value::Num(y))) => {
                        Expr::Literal(Value::Num(x * y))
                    }
                    _ => expr.clone(),
                }
            }
            Expr::Div(l, r) => {
                let (l, r) = (Self::optimize_expr(l), Self::optimize_expr(r));
                match (l, r) {
                    (Expr::Literal(Value::Num(x)), Expr::Literal(Value::Num(y))) => {
                        Expr::Literal(Value::Num(x / y))
                    }
                    _ => expr.clone(),
                }
            }
            Expr::Not(expr) => Self::optimize_expr(expr),
            Expr::EqualEqual(l, r) => {
                let (l, r) = (Self::optimize_expr(l), Self::optimize_expr(r));
                match (&l, &r) {
                    (Expr::Literal(l_val), Expr::Literal(r_val)) => match (l_val, r_val) {
                        (Value::Bool(l), Value::Bool(r)) => Expr::Literal(Value::Bool(l == r)),
                        (Value::Num(l), Value::Num(r)) => Expr::Literal(Value::Bool(l == r)),
                        (Value::String(l), Value::String(r)) => Expr::Literal(Value::Bool(l == r)),
                        (Value::Array(l), Value::Array(r)) => Expr::Literal(Value::Bool(l == r)),
                        _ => expr.clone(),
                    },
                    _ => expr.clone(),
                }
            }
            Expr::And(l, r) => {
                let (l, r) = (Self::optimize_expr(l), Self::optimize_expr(r));
                match (l, r) {
                    (Expr::Literal(l), Expr::Literal(r)) => {
                        Expr::Literal(Value::Bool(l.is_truthy() && r.is_truthy()))
                    }
                    _ => expr.clone(),
                }
            }
            Expr::Or(l, r) => {
                let (l, r) = (Self::optimize_expr(l), Self::optimize_expr(r));
                match (l, r) {
                    (Expr::Literal(l), Expr::Literal(r)) => {
                        Expr::Literal(Value::Bool(l.is_truthy() || r.is_truthy()))
                    }
                    _ => expr.clone(),
                }
            }
            Expr::Var(_) | Expr::Call(..) | Expr::FnBody(_) => expr.clone(),
        }
    }
}
