use std::collections::HashMap;
use crate::reader::token::Token;

#[derive(Hash, Clone, Copy)]
pub enum NodeType {
    Program,
    Statement,
    Expression,
    P,
    T,
    Var,
    Lit
}

impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        matches!(self, other)
    }
}

pub struct Node {
    token: Token,
    node_type: NodeType,
    params: HashMap<String, String>,
    childs: Vec<Box<Node>>,
}

impl Node {
    pub fn new(node_type: NodeType, token: Token) -> Self {
        Node {
            token,
            node_type,
            params: HashMap::new(),
            childs: Vec::new(),
        }
    }
}
