use std::{error::Error, process::exit};

use serde::{Deserialize, Serialize};

use crate::{expr::Expr, stmt::Stmt, value::Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Bytecode {
    Print,
    Add,
    Sub,
    Mul,
    Div,
    UnaryPlus,
    UnaryMinus,
    Exit,
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
                Stmt::Exit(expr) => {
                    bytecode.extend(Self::eval_expr(expr));
                    bytecode.push(Bytecode::Exit);
                }
                Stmt::Expr(expr) => bytecode.extend(Self::eval_expr(expr)),
                Stmt::If(_, _) => todo!(),
                Stmt::Block(_) => todo!(),
                Stmt::Assign(_, _) => todo!(),
                Stmt::Func(_, _, _) => todo!(),
                Stmt::Return(_) => todo!(),
                Stmt::While(_, _) => todo!(),
            }
        }
        bytecode
    }

    fn eval_expr(expr: &Expr) -> Vec<Bytecode> {
        match expr {
            Expr::Literal(value) => vec![Bytecode::Literal(value.clone())],
            Expr::Add(x, y) => Self::bin_op(x, y, Bytecode::Add),
            Expr::Sub(x, y) => Self::bin_op(x, y, Bytecode::Sub),
            Expr::Mul(x, y) => Self::bin_op(x, y, Bytecode::Mul),
            Expr::Div(x, y) => Self::bin_op(x, y, Bytecode::Div),
            Expr::UnaryPlus(x) => Self::unary_op(x, Bytecode::UnaryPlus),
            Expr::UnaryMinus(x) => Self::unary_op(x, Bytecode::UnaryMinus),
            Expr::AddAssign(_, _) => todo!(),
            Expr::Not(_) => todo!(),
            Expr::NotEqual(_, _) => todo!(),
            Expr::EqualEqual(_, _) => todo!(),
            Expr::LessThan(_, _) => todo!(),
            Expr::LessThanEqual(_, _) => todo!(),
            Expr::GreaterThan(_, _) => todo!(),
            Expr::GreaterThanEqual(_, _) => todo!(),
            Expr::And(_, _) => todo!(),
            Expr::Or(_, _) => todo!(),
            Expr::Var(_) => todo!(),
            Expr::Call(_, _) => todo!(),
            Expr::FnBody(_) => todo!(),
        }
    }

    fn bin_op(x: &Expr, y: &Expr, bc: Bytecode) -> Vec<Bytecode> {
        let mut res = vec![];
        res.extend(Self::eval_expr(x));
        res.extend(Self::eval_expr(y));
        res.push(bc);
        res
    }
    fn unary_op(x: &Expr, bc: Bytecode) -> Vec<Bytecode> {
        let mut res = vec![];
        res.extend(Self::eval_expr(x));
        res.push(bc);
        res
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

    fn pop_two(&mut self) -> (Value, Value) {
        let y = self.stack.pop().expect("No item to add on stack");
        let x = self.stack.pop().expect("No item to add on stack");
        (x, y)
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("No item to add on stack")
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
                    let (x, y) = self.pop_two();
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
                    let (x, y) = self.pop_two();
                    match (x, y) {
                        (Value::Num(x), Value::Num(y)) => {
                            self.stack.push(Value::Num(x - y));
                        }
                        _ => panic!("Cannot sub operands"),
                    }
                }
                Bytecode::Mul => {
                    let (x, y) = self.pop_two();
                    match (x, y) {
                        (Value::Num(x), Value::Num(y)) => {
                            self.stack.push(Value::Num(x * y));
                        }
                        _ => panic!("Cannot mul operands"),
                    }
                }
                Bytecode::Div => {
                    let (x, y) = self.pop_two();
                    match (x, y) {
                        (Value::Num(x), Value::Num(y)) => {
                            self.stack.push(Value::Num(x / y));
                        }
                        _ => panic!("Cannot div operands"),
                    }
                }
                Bytecode::UnaryPlus => {
                    let x = self.pop();
                    match x {
                        Value::Num(x) => {
                            self.stack.push(Value::Num(x.abs()));
                        }
                        _ => panic!("Cannot pos operand"),
                    }
                }
                Bytecode::UnaryMinus => {
                    let x = self.pop();
                    match x {
                        Value::Num(x) => {
                            self.stack.push(Value::Num(-x));
                        }
                        _ => panic!("Cannot negate operand"),
                    }
                }
                Bytecode::Literal(value) => self.stack.push(value.clone()),
                Bytecode::Exit => {
                    let x = self.pop();
                    match x {
                        Value::Num(n) => exit(n as i32),
                        _ => panic!("Cannot exit with non-number."),
                    }
                }
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
