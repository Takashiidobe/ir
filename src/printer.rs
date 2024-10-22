use std::fmt;

use crate::stmt::Stmt;

pub struct Printer {
    body: Vec<Stmt>,
}

impl Printer {
    pub fn new(body: &[Stmt]) -> Self {
        Self {
            body: body.to_vec(),
        }
    }
}

impl fmt::Display for Printer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut body = String::default();
        for instruction in &self.body {
            body.push_str(&indent_stmt(instruction, 0));
        }
        body.pop();
        f.write_str(&body)
    }
}

fn indent_stmt(stmt: &Stmt, indent: usize) -> String {
    let mut s = String::new();
    for _ in 0..indent {
        s.push('\t');
    }
    match stmt {
        Stmt::Block(stmts) => {
            s.push('{');
            s.push('\n');
            for stmt in stmts {
                s.push_str(&indent_stmt(stmt, indent + 1));
            }
            for _ in 0..indent {
                s.push('\t');
            }
            s.push('}');
            s.push('\n');
        }
        _ => {
            s.push_str(&stmt.to_string());
            s.push('\n');
        }
    }
    s
}
