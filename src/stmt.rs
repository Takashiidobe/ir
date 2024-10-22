use std::fmt;

use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    Exit(Expr),
    Print(Expr),
    Expr(Expr),
    If(Expr, Vec<Stmt>),
    Block(Vec<Stmt>),
    Assign(String, Expr),
    Func(String, Vec<String>, Vec<Stmt>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Exit(code) => f.write_fmt(format_args!("exit({})", code)),
            Stmt::Print(num) => f.write_fmt(format_args!("print({})", num)),
            Stmt::Expr(expr) => f.write_fmt(format_args!("{};", expr)),
            Stmt::If(cond, body) => {
                let mut s = String::new();
                for stmt in body {
                    s.push_str(&stmt.to_string());
                    s.push('\n');
                }
                s.pop();
                f.write_fmt(format_args!("if {cond} {{ {s} }}"))
            }
            Stmt::Block(stmts) => {
                let mut s = String::from("{ ");
                for stmt in stmts {
                    s.push_str(&stmt.to_string());
                    s.push('\n');
                }
                s.pop();
                s.push_str(" }");
                f.write_str(&s)
            }
            Stmt::Assign(name, expr) => f.write_fmt(format_args!("let {name} = {expr};")),
            Stmt::Func(name, args, body) => {
                let mut s = format!("fn {name}(");
                for arg in args {
                    s.push_str(arg);
                    s.push_str(", ");
                }
                s.pop();
                s.pop();
                s.push_str(") {\n");
                for stmt in body {
                    s.push('\t');
                    s.push_str(&stmt.to_string());
                    s.push('\n');
                }
                s.push_str("}");
                f.write_str(&s)
            }
        }
    }
}
