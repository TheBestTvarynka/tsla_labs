use std::collections::HashMap;
use crate::reader::token::{Token, Value, TokenType};
use crate::reader::lexer::GeneratedTable;

#[derive(Hash, Clone, Copy, Debug)]
pub enum NodeType {
    Program,
    Statement,
    Expression,
    P,
    T,
    Var,
    Lit
}

impl NodeType {
    pub fn to_string(node_type: NodeType) -> String {
        match node_type {
            NodeType::Program => "Program".to_owned(),
            NodeType::Statement => "Statement".to_owned(),
            NodeType::Expression => "Expression".to_owned(),
            NodeType::P => "P".to_owned(),
            NodeType::T => "T".to_owned(),
            NodeType::Var => "Var".to_owned(),
            NodeType::Lit => "Lit".to_owned(),
        }
    }
}

impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        matches!(self, other)
    }
}

#[derive(Debug)]
pub struct Node {
    pub token: Token,
    pub name: String,
    pub node_type: NodeType,
    pub params: HashMap<String, String>,
    pub childs: Vec<Box<Node>>,
}

impl Node {
    pub fn new(node_type: NodeType, token: Token) -> Self {
        Node {
            token,
            node_type,
            name: NodeType::to_string(node_type),
            params: HashMap::new(),
            childs: Vec::new(),
        }
    }

    pub fn temp(token: Token) -> Self {
        let name;
        match token.clone().value {
            Value::Value(value) => name = value,
            _ => name = "<name>".to_owned(),
        };
        Node {
            token,
            node_type: NodeType::Lit,
            name,
            params: HashMap::new(),
            childs: Vec::new(),
        }
    }

    pub fn from_token(token: Token, table: &GeneratedTable) -> Self {
        let name = match token.clone().token_type {
            TokenType::F32Literal => "Lit".to_owned(),
            TokenType::I42Literal => "Lit".to_owned(),
            TokenType::StringLiteral => "Lit".to_owned(),
            TokenType::U32Literal => "Lit".to_owned(),
            TokenType::Name => "Name".to_owned(),
            _ => token.clone().get_value(table),
        };
        Node {
            token,
            name,
            childs: Vec::new(),
            params: HashMap::new(),
            node_type: NodeType::T,
        }
    }
}
