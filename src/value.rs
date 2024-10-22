use std::fmt;

use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Bool(bool),
    Num(i64),
    String(String),
    Array(Vec<Expr>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(n) => f.write_str(&n.to_string()),
            Value::String(s) => f.write_fmt(format_args!("\"{}\"", &s)),
            Value::Array(arr) => {
                let mut res = String::from("[");
                for item in arr {
                    res.push_str(&item.to_string());
                    res.push_str(", ");
                }
                res.pop();
                res.pop();
                res.push(']');
                f.write_str(&res)
            }
            Value::Bool(b) => f.write_str(&b.to_string()),
            Value::Null => f.write_str("null"),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Num(n) => *n != 0,
            Value::String(s) => !s.is_empty(),
            Value::Array(vec) => !vec.is_empty(),
            Value::Null => false,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}
