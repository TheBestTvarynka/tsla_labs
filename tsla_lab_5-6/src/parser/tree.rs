use std::collections::HashMap;
use crate::reader::token::{Token, Value, TokenType};
use crate::reader::lexer::GeneratedTable;

#[derive(Hash, Clone, Debug)]
pub enum NodeType {
    Program,
    Statement,
    Expression(String),
    P(String),
    T(String),
    Var,
    Lit(String),
}

impl NodeType {
    pub fn to_string(node_type: NodeType) -> String {
        match node_type {
            NodeType::Program => "Program".to_owned(),
            NodeType::Statement => "Statement".to_owned(),
            NodeType::Expression(_) => "Expression".to_owned(),
            NodeType::P(_) => "P".to_owned(),
            NodeType::T(_) => "T".to_owned(),
            NodeType::Var => "Var".to_owned(),
            NodeType::Lit(_) => "Lit".to_owned(),
        }
    }
}

impl PartialEq for NodeType {
    fn eq(&self, _other: &Self) -> bool {
        matches!(self, _other)
    }
}

#[derive(Debug, Clone)]
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
            node_type: node_type.clone(),
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
            node_type: NodeType::Lit("".to_owned()),
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
            token: token.clone(),
            name,
            childs: Vec::new(),
            params: HashMap::new(),
            node_type: NodeType::Lit(token.get_value(table)),
        }
    }

    pub fn print(node: Node) {
        let mut data = vec![vec![Box::new(node)]];
        let mut flag = true;
        while flag {
            flag = false;
            let mut new_data = Vec::new();
            for nodes in data {
                print!("|");
                for node in nodes {
                    print!(" {:?} ", &node.node_type);
                    if node.childs.len() > 0 {
                        flag = true;
                    }
                    new_data.push(node.childs);
                }
                print!("|");
            }
            println!("");
            data = new_data;
        }
    }
}
