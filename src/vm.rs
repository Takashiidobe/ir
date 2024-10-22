use std::{cell::RefCell, collections::HashMap, process::exit, rc::Rc};

use crate::{error::EvalError, expr::Expr, stmt::Stmt, value::Value};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VM {
    pub body: Vec<Stmt>,
    pub vars: Env<Value>,
    pub fns: Env<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Function {
    args: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env<V> {
    parent: Option<Rc<RefCell<Env<V>>>>,
    values: HashMap<String, V>,
}

impl<V: Clone> Env<V> {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn from(parent: &Rc<RefCell<Env<V>>>) -> Self {
        Env {
            parent: Some(Rc::clone(parent)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: V) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Result<V, EvalError> {
        if let Some(value) = self.values.get(key) {
            Ok((*value).clone())
        } else if let Some(ref enclosing) = self.parent {
            enclosing.borrow().get(key)
        } else {
            Err(EvalError::Error(format!("Undefined variable '{}'", key)))
        }
    }

    pub fn assign(&mut self, key: &str, value: V) -> Result<(), EvalError> {
        if self.values.contains_key(key) {
            self.values.insert(key.to_string(), value);
            Ok(())
        } else if let Some(ref enclosing) = self.parent {
            enclosing.borrow_mut().assign(key, value)
        } else {
            Err(EvalError::Error(format!("Undefined variable '{}'", key)))
        }
    }
}

impl<V: Clone> Default for Env<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn eval(&mut self, instructions: &[Stmt]) -> Result<Self, EvalError> {
        for stmt in instructions {
            match stmt {
                Stmt::Exit(expr) => {
                    let expr = self.eval_expr(expr)?;
                    match expr {
                        Expr::Literal(Value::Num(n)) => exit(n as i32),
                        _ => {
                            EvalError::Error(format!("Gave the wrong type {expr} to exit"));
                        }
                    }
                }
                Stmt::Print(expr) => {
                    let expr = self.eval_expr(expr)?;
                    println!("{}", expr);
                }
                Stmt::Expr(expr) => {
                    self.eval_expr(expr)?;
                }
                Stmt::If(cond, body) => {
                    let cond = self.eval_expr(cond)?;
                    if let Expr::Literal(value) = cond {
                        if value.is_truthy() {
                            *self = Self::eval(self, body)?.clone();
                        }
                    }
                }
                Stmt::Block(stmts) => {
                    let old_vars = self.vars.clone();
                    self.vars = Env::from(&Rc::new(RefCell::new(self.vars.clone())));
                    self.vars.values = HashMap::default();
                    *self = Self::eval(self, stmts)?.clone();
                    self.vars = old_vars;
                }
                Stmt::Assign(s, expr) => {
                    let expr = self.eval_expr(expr)?;
                    if let Expr::Literal(value) = expr {
                        self.vars.define(s, value);
                    }
                }
                Stmt::Func(name, args, body) => self.fns.define(
                    name,
                    Function {
                        args: args.to_vec(),
                        body: body.to_vec(),
                    },
                ),
            }
        }
        self.body.extend(instructions.to_vec());
        Ok(self.clone())
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Expr, EvalError> {
        match expr {
            Expr::Literal(l) => match l {
                Value::Bool(_) | Value::Num(_) | Value::String(_) | Value::Null => Ok(expr.clone()),
                Value::Array(vec) => {
                    let mut items = vec![];
                    for expr in vec {
                        items.push(self.eval_expr(expr)?);
                    }
                    Ok(Expr::Literal(Value::Array(items)))
                }
            },
            Expr::Add(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                match (left, right) {
                    (Expr::Literal(Value::Num(l)), Expr::Literal(Value::Num(r))) => {
                        Ok(Expr::Literal(Value::Num(l + r)))
                    }
                    (Expr::Literal(Value::String(x)), Expr::Literal(Value::String(y))) => {
                        let mut res = x.to_string();
                        res.push_str(&y);
                        Ok(Expr::Literal(Value::String(res)))
                    }
                    (l, r) => Err(EvalError::InvalidBinaryExpr(l, "+".to_string(), r)),
                }
            }
            Expr::Sub(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);

                match (left, right) {
                    (Expr::Literal(Value::Num(l)), Expr::Literal(Value::Num(r))) => {
                        Ok(Expr::Literal(Value::Num(l - r)))
                    }
                    (l, r) => Err(EvalError::InvalidBinaryExpr(l, "-".to_string(), r)),
                }
            }
            Expr::Mul(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                match (left, right) {
                    (Expr::Literal(Value::Num(l)), Expr::Literal(Value::Num(r))) => {
                        Ok(Expr::Literal(Value::Num(l * r)))
                    }
                    (l, r) => Err(EvalError::InvalidBinaryExpr(l, "*".to_string(), r)),
                }
            }
            Expr::Div(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                match (left, right) {
                    (Expr::Literal(Value::Num(l)), Expr::Literal(Value::Num(r))) => {
                        Ok(Expr::Literal(Value::Num(l / r)))
                    }
                    (l, r) => Err(EvalError::InvalidBinaryExpr(l, "/".to_string(), r)),
                }
            }
            Expr::Not(b) => {
                let b = self.eval_expr(b)?;
                match b {
                    Expr::Literal(Value::Bool(b)) => Ok(Expr::Literal(Value::Bool(!b))),
                    _ => Err(EvalError::InvalidUnaryExpr("!".to_string(), b)),
                }
            }
            Expr::EqualEqual(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                match (&left, &right) {
                    (Expr::Literal(l_val), Expr::Literal(r_val)) => match (l_val, r_val) {
                        (Value::Bool(l), Value::Bool(r)) => Ok(Expr::Literal(Value::Bool(l == r))),
                        (Value::Num(l), Value::Num(r)) => Ok(Expr::Literal(Value::Bool(l == r))),
                        (Value::String(l), Value::String(r)) => {
                            Ok(Expr::Literal(Value::Bool(l == r)))
                        }
                        (Value::Array(l), Value::Array(r)) => {
                            Ok(Expr::Literal(Value::Bool(l == r)))
                        }
                        _ => Err(EvalError::InvalidBinaryExpr(left, "==".to_string(), right)),
                    },
                    _ => Ok(Expr::Literal(Value::Bool(false))),
                }
            }
            Expr::And(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);

                match (left, right) {
                    (Expr::Literal(l), Expr::Literal(r)) => {
                        Ok(Expr::Literal(Value::Bool(l.is_truthy() && r.is_truthy())))
                    }
                    _ => Ok(Expr::Literal(Value::Bool(false))),
                }
            }
            Expr::Or(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                match (left, right) {
                    (Expr::Literal(l), Expr::Literal(r)) => {
                        Ok(Expr::Literal(Value::Bool(l.is_truthy() || r.is_truthy())))
                    }
                    _ => Ok(Expr::Literal(Value::Bool(false))),
                }
            }
            Expr::Var(name) => match self.vars.get(name) {
                Ok(val) => Ok(Expr::Literal(val.clone())),
                Err(e) => Err(e),
            },
            Expr::Call(name, args) => {
                // get the function itself
                let body = &self.fns.get(name)?;
                let old_vars = self.vars.clone();
                self.vars = Env::from(&Rc::new(RefCell::new(self.vars.clone())));
                self.vars.values = HashMap::default();
                for i in 0..args.len() {
                    let arg = self.eval_expr(&args[i])?;
                    match arg {
                        Expr::Literal(value) => {
                            self.vars.define(&body.args[i], value);
                        }
                        _ => return Err(EvalError::Error("Arg was not valid".to_string())),
                    }
                }
                *self = Self::eval(self, &body.body)?.clone();
                self.vars = old_vars;

                Ok(Expr::Literal(Value::Null))
            }
        }
    }
}