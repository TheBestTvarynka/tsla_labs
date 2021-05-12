use crate::parser::tree::{Node, NodeType};
use crate::reader::token::TokenType;

pub fn lint(node: Box<Node>) -> Result<(Box<Node>, String), String> {
    let mut err = Vec::new();
    let mut t = "unknow".to_owned();
    match node.clone().node_type {
        NodeType::Program => {
            let node = node.clone();
            println!("in node: {}", node.name);
            println!("childs len: {}", node.childs.len());
            for node in node.childs {
                match lint(node) {
                    Result::Ok((_, _)) => {},
                    Result::Err(err_msg) => err.push(err_msg),
                }
            };
            t = "Program".to_owned();
        },
        NodeType::Statement => {
            let node = node.clone();
            println!("in node: {}", node.name);
            println!("childs len: {}", node.childs.len());
            match lint(node.childs[1].clone()) {
                Result::Ok(_) => {},
                Result::Err(err_msg) => err.push(err_msg),
            };
            t = "Statement".to_owned();
        },
        NodeType::Expression(val) => {
            let node = node.clone();
            println!("in node: {} - {}", node.name, val);
            println!("childs len: {}", node.childs.len());
            let mut ts = Vec::new();
            if node.childs.len() == 1 {
                return lint(node.childs[0].clone());
            } else {
                for node in node.childs {
                    match lint(node) {
                        Result::Ok((_, t)) => ts.push(t),
                        Result::Err(err_msg) => err.push(err_msg),
                    }
                }
                println!("types: {:?}", ts);
                for i in 0..(ts.len() - 1) {
                    if ts[i] != ts[i + 1] {
                        err.push(format!("types do not match: {} and {} at ({}, {})", ts[i], ts[i + 1], node.token.line, node.token.position));
                    }
                }
                t = ts[0].clone();
            }
        },
        NodeType::P(val) => {
            let node = node.clone();
            println!("in node: {} - {}", node.name, val);
            println!("childs len: {}", node.childs.len());
            let mut ts = Vec::new();
            if node.childs.len() == 1 {
                return lint(node.childs[0].clone());
            } else {
                for node in node.childs {
                    match lint(node) {
                        Result::Ok((_, t)) => ts.push(t),
                        Result::Err(err_msg) => err.push(err_msg),
                    }
                }
                println!("types: {:?}", ts);
                for i in 0..(ts.len() - 1) {
                    if ts[i] != ts[i + 1] {
                        err.push(format!("types do not match: {} and {} at ({}, {})", ts[i], ts[i + 1], node.token.line, node.token.position));
                    }
                }
                t = ts[0].clone();
            }
        },
        NodeType::T(val) => {
            let node = node.clone();
            println!("in node: {} - {}", node.name, val);
            println!("childs len: {}", node.childs.len());
            let mut ts = Vec::new();
            if node.childs.len() == 1 {
                return lint(node.childs[0].clone());
            } else {
                for node in node.childs {
                    match lint(node) {
                        Result::Ok((_, t)) => ts.push(t),
                        Result::Err(err_msg) => err.push(err_msg),
                    }
                }
                println!("types: {:?}", ts);
                if ts.len() != 0 {
                    for i in 0..(ts.len() - 1) {
                        if ts[i] != ts[i + 1] {
                            err.push(format!("types do not match: {} and {} at ({}, {})", ts[i], ts[i + 1], node.token.line, node.token.position));
                        }
                    }
                    t = ts[0].clone();
                }
            }
        },
        NodeType::Lit(val) => {
            println!("----------------------");
            let node = node.clone();
            println!("in node: {} - {}", node.name, val);
            println!("childs len: {}", node.childs.len());
            println!("{:?}", &node);
            t = TokenType::to_string(node.token.token_type);
        },
        NodeType::Var => {},
    };
    if err.len() != 0 {
        Result::Err(err.join("\n"))
    } else {
        Result::Ok((node, t))
    }
}