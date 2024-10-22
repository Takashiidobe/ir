use std::fmt;

use crate::{stmt::Stmt, value::Value};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Literal(Value),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    EqualEqual(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Var(String),
    Call(String, Vec<Expr>),
    FnBody(Vec<Stmt>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(l) => f.write_str(&l.to_string()),
            Expr::Add(x, y) => f.write_fmt(format_args!("{} + {}", x, y)),
            Expr::Sub(x, y) => f.write_fmt(format_args!("{} - {}", x, y)),
            Expr::Mul(x, y) => f.write_fmt(format_args!("{} * {}", x, y)),
            Expr::Div(x, y) => f.write_fmt(format_args!("{} / {}", x, y)),
            Expr::Not(val) => f.write_fmt(format_args!("!{}", val)),
            Expr::EqualEqual(x, y) => f.write_fmt(format_args!("{} == {}", x, y)),
            Expr::And(x, y) => f.write_fmt(format_args!("{} && {}", x, y)),
            Expr::Or(x, y) => f.write_fmt(format_args!("{} || {}", x, y)),
            Expr::Var(name) => f.write_str(name),
            Expr::Call(name, args) => {
                let mut s = format!("{name}(");
                for arg in args {
                    s.push_str(&arg.to_string());
                    s.push_str(", ");
                }
                s.pop();
                s.pop();
                s.push_str(")");
                f.write_str(&s)
            }
            Expr::FnBody(body) => f.write_fmt(format_args!("{body:?}")),
        }
    }
}

impl From<bool> for Expr {
    fn from(value: bool) -> Self {
        Expr::Literal(Value::Bool(value))
    }
}

impl From<bool> for Box<Expr> {
    fn from(value: bool) -> Self {
        Box::new(Expr::Literal(Value::Bool(value)))
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::Literal(Value::Num(value))
    }
}

impl From<i64> for Box<Expr> {
    fn from(value: i64) -> Self {
        Box::new(Expr::Literal(Value::Num(value)))
    }
}

impl From<&str> for Box<Expr> {
    fn from(value: &str) -> Self {
        Box::new(Expr::Literal(Value::String(value.to_string())))
    }
}

impl From<&str> for Expr {
    fn from(value: &str) -> Self {
        Expr::Literal(Value::String(value.to_string()))
    }
}

impl From<&[Expr]> for Box<Expr> {
    fn from(value: &[Expr]) -> Self {
        Box::new(Expr::Literal(Value::Array(value.to_vec())))
    }
}

impl From<&[Expr]> for Expr {
    fn from(value: &[Expr]) -> Self {
        Expr::Literal(Value::Array(value.to_vec()))
    }
}

impl From<Vec<Expr>> for Box<Expr> {
    fn from(value: Vec<Expr>) -> Self {
        Box::new(Expr::Literal(Value::Array(value)))
    }
}

impl From<Vec<Expr>> for Expr {
    fn from(value: Vec<Expr>) -> Self {
        Expr::Literal(Value::Array(value))
    }
}
