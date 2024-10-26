use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{expr::Expr, stmt::Stmt, value::Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Bytecode {
    Print,
    Add,
    Sub,
    Literal(Value),
}

#[derive(Default, Debug, Clone)]
pub struct Compiler;

impl Compiler {
    pub fn compile(&mut self, stmts: &[Stmt]) -> Vec<Bytecode> {
        let mut bytecode = vec![];
        for stmt in stmts {
            match stmt {
                Stmt::Print(expr) => {
                    bytecode.extend(Self::eval_expr(expr));
                    bytecode.push(Bytecode::Print);
                }
                _ => todo!(),
            }
        }
        bytecode
    }

    fn eval_expr(expr: &Expr) -> Vec<Bytecode> {
        match expr {
            Expr::Literal(value) => vec![Bytecode::Literal(value.clone())],
            Expr::Add(x, y) => {
                let mut res = vec![];
                res.extend(Self::eval_expr(x));
                res.extend(Self::eval_expr(y));
                res.push(Bytecode::Add);
                res
            }
            Expr::Sub(x, y) => {
                let mut res = vec![];
                res.extend(Self::eval_expr(x));
                res.extend(Self::eval_expr(y));
                res.push(Bytecode::Sub);
                res
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VM<W: std::io::Write> {
    stack: Vec<Value>,
    writer: W,
}

impl<W: std::io::Write> VM<W> {
    pub fn new(writer: W) -> Self {
        Self {
            stack: vec![],
            writer,
        }
    }

    pub fn eval(&mut self, bytecodes: &[Bytecode]) -> Result<(), Box<dyn Error>> {
        for bc in bytecodes {
            match bc {
                Bytecode::Print => {
                    let val = self.stack.pop().expect("Could not print value");
                    match val {
                        Value::Num(n) => self.writer.write_all(&n.to_ne_bytes())?,
                        Value::String(s) => self.writer.write_all(&s.into_bytes())?,
                        _ => todo!(),
                    }
                }
                Bytecode::Add => {
                    let y = self.stack.pop().expect("No item to add on stack");
                    let x = self.stack.pop().expect("No item to add on stack");
                    match (x, y) {
                        (Value::Num(x), Value::Num(y)) => {
                            self.stack.push(Value::Num(x + y));
                        }
                        (Value::String(mut x), Value::String(y)) => {
                            x.push_str(&y);
                            self.stack.push(Value::String(x));
                        }
                        _ => panic!("Cannot add operands"),
                    }
                }
                Bytecode::Sub => {
                    let y = self.stack.pop().expect("No item to sub on stack");
                    let x = self.stack.pop().expect("No item to sub on stack");
                    match (x, y) {
                        (Value::Num(x), Value::Num(y)) => {
                            self.stack.push(Value::Num(x - y));
                        }
                        _ => panic!("Cannot sub operands"),
                    }
                }
                Bytecode::Literal(value) => self.stack.push(value.clone()),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use insta::assert_yaml_snapshot as test;

    use crate::{
        bytecode::{Compiler, VM},
        expr::Expr,
        stmt::Stmt,
    };

    #[test]
    fn test() {
        let ast = vec![
            Stmt::Print(Expr::Add(
                Box::new(Expr::Add(1.into(), 2.into())),
                Box::new(Expr::Add(3.into(), 4.into())),
            )),
            Stmt::Print(Expr::Sub(
                Box::new(Expr::Sub(10.into(), 0.into())),
                Box::new(Expr::Sub(5.into(), 0.into())),
            )),
        ];

        let mut compiler = Compiler;

        test!(compiler.compile(&ast));
    }

    #[test]
    fn test_vm() -> Result<(), Box<dyn Error>> {
        let ast = vec![
            Stmt::Print(Expr::Add(
                Box::new(Expr::Add(1.into(), 2.into())),
                Box::new(Expr::Add(3.into(), 4.into())),
            )),
            Stmt::Print(Expr::Sub(
                Box::new(Expr::Sub(10.into(), 0.into())),
                Box::new(Expr::Sub(5.into(), 0.into())),
            )),
        ];

        let mut compiler = Compiler;

        let bc = compiler.compile(&ast);
        let mut buf = vec![];
        VM::new(&mut buf).eval(&bc)?;

        test!(buf);
        Ok(())
    }

    use arbtest::arbtest;

    #[test]
    fn no_crash() {
        arbtest(|input| {
            let ast: Vec<Stmt> = input.arbitrary().unwrap();
            let mut compiler = Compiler;

            let bc = compiler.compile(&ast);
            let mut buf = vec![];
            match VM::new(&mut buf).eval(&bc) {
                Ok(_) => Ok(()),
                Err(_) => Err(arbitrary::Error::IncorrectFormat),
            }
        });
    }
}
