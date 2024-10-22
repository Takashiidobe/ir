use crate::stmt::Stmt;

pub struct Serdes;

impl Serdes {
    pub fn serialize(stmts: Vec<Stmt>) -> Vec<u8> {
        bincode::serialize(&stmts).unwrap()
    }

    pub fn deserialize(bytes: &[u8]) -> Vec<Stmt> {
        bincode::deserialize(bytes).unwrap()
    }
}
