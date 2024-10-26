use std::{cell::RefCell, collections::HashMap, process::exit, rc::Rc};

use crate::{error::EvalError, expr::Expr, stmt::Stmt, value::Value};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VM {
    pub body: Vec<Stmt>,
    pub vars: Env<Value>,
    pub fns: Env<Function>,
    pub in_fn: bool,
    pub return_val: Option<Value>,
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
            self.body.push(stmt.clone());
            self.eval_stmt(stmt)?;
        }
        Ok(self.clone())
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), EvalError> {
        match stmt {
            Stmt::Exit(expr) => {
                let expr = self.eval_expr(expr)?;
                match expr {
                    Expr::Literal(Value::Num(n)) => exit(n as i32),
                    _ => {
                        return Err(EvalError::Error(format!(
                            "Gave the wrong type {expr} to exit"
                        )))
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
                *self = self.eval(stmts)?.clone();
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
            Stmt::Return(expr) => {
                let expr = self.eval_expr(expr)?;
                if self.in_fn {
                    self.return_val = match expr {
                        Expr::Literal(value) => Some(value),
                        _ => unreachable!(),
                    }
                }
            }
            Stmt::While(cond, body) => loop {
                let cond = self.eval_expr(cond)?;
                if cond == Expr::Literal(Value::Bool(false)) {
                    break;
                }
                *self = self.eval(body)?.clone();
            },
        }
        Ok(())
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
                Ok((left == right).into())
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
                for (i, _) in args.iter().enumerate() {
                    let arg = self.eval_expr(&args[i])?;
                    match arg {
                        Expr::Literal(value) => {
                            self.vars.define(&body.args[i], value);
                        }
                        _ => return Err(EvalError::Error("Arg was not valid".to_string())),
                    }
                }
                let res = self.eval_expr(&Expr::FnBody(body.body.clone()))?.clone();
                self.vars = old_vars;
                Ok(res)
            }
            Expr::FnBody(body) => {
                self.in_fn = true;
                for stmt in body {
                    let mut ret_val: Option<Expr> = None;
                    if let Some(saved_val) = &self.return_val {
                        ret_val = Some(Expr::Literal(saved_val.clone()));
                    }
                    if ret_val.is_some() {
                        self.return_val = None;
                        return Ok(ret_val.unwrap());
                    }
                    self.eval_stmt(stmt)?;
                }
                self.in_fn = false;

                Ok(Expr::Literal(Value::Null))
            }
            Expr::UnaryPlus(expr) => {
                let expr = self.eval_expr(expr)?;
                match expr {
                    Expr::Literal(Value::Num(n)) => Ok(Expr::Literal(Value::Num(n.abs()))),
                    _ => unreachable!(),
                }
            }
            Expr::UnaryMinus(expr) => {
                let expr = self.eval_expr(expr)?;
                match expr {
                    Expr::Literal(Value::Num(n)) => Ok(Expr::Literal(Value::Num(-n))),
                    _ => unreachable!(),
                }
            }
            Expr::NotEqual(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                Ok((left != right).into())
            }
            Expr::LessThan(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                Ok((left < right).into())
            }
            Expr::LessThanEqual(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                Ok((left <= right).into())
            }
            Expr::GreaterThan(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                Ok((left > right).into())
            }
            Expr::GreaterThanEqual(x, y) => {
                let (left, right) = (self.eval_expr(x)?, self.eval_expr(y)?);
                Ok((left >= right).into())
            }
            Expr::AddAssign(var, incr) => {
                let incr = self.eval_expr(incr)?;
                let count = match incr {
                    Expr::Literal(Value::Num(n)) => n,
                    _ => unreachable!(), // crashes currently, have to handle properly.
                };
                match **var {
                    Expr::Var(ref name) => match self.vars.get(&name) {
                        Ok(val) => match val {
                            Value::Num(n) => {
                                self.vars.define(&name, Value::Num(n + count));
                                Ok(Expr::Literal(self.vars.get(&name)?))
                            }
                            _ => unreachable!(),
                        },
                        Err(e) => Err(e),
                    },
                    _ => todo!(),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{stmt::Stmt, vm::VM};
    use arbtest::arbtest;

    #[test]
    fn no_crash() {
        arbtest(|input| {
            let stmts: Vec<Stmt> = input.arbitrary()?;
            let mut vm = VM::default();
            match vm.eval(&stmts) {
                Ok(_) => Ok(()),
                Err(_) => Err(arbitrary::Error::NotEnoughData),
            }
        });
    }
}
