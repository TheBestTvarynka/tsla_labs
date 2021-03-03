use std::fs::{File};
use std::io::prelude::*;
use crate::lexer::Token;

#[derive(Debug, Clone)]
enum Expression {
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    BinaryMathOperation(String, Box<Expression>, Box<Expression>),
    Value(u32)
}

fn get_priority(token: Token) -> Option<u8> {
    match token {
        Token::Plus => Option::Some(2),
        Token::Minus => Option::Some(2),
        Token::Multiply => Option::Some(3),
        Token::Devide => Option::Some(3),
        Token::Less => Option::Some(1),
        Token::Greater => Option::Some(1),
        Token::Equal => Option::Some(1),
        Token::NotEqual => Option::Some(1),
        Token::OpenParenthesis => Option::Some(0),
        Token::CloseParenthesis => Option::Some(0),
        _ => Option::None,
    }
}

fn build_node(token: Token, left: Expression, right: Expression) -> Expression {
    match token {
        Token::Plus => Expression::BinaryMathOperation("+".to_owned(), Box::new(left), Box::new(right)),
        Token::Minus => Expression::BinaryMathOperation("-".to_owned(), Box::new(left), Box::new(right)),
        Token::Multiply => Expression::BinaryMathOperation("*".to_owned(), Box::new(left), Box::new(right)),
        Token::Devide => Expression::BinaryMathOperation("/".to_owned(), Box::new(left), Box::new(right)),
        Token::Less => Expression::BinaryMathOperation("<".to_owned(), Box::new(left), Box::new(right)),
        Token::Greater => Expression::BinaryMathOperation(">".to_owned(), Box::new(left), Box::new(right)),
        Token::Equal => Expression::BinaryMathOperation("==".to_owned(), Box::new(left), Box::new(right)),
        Token::NotEqual => Expression::BinaryMathOperation("!=".to_owned(), Box::new(left), Box::new(right)),
        _ => panic!("Unsupported math operator {:?}", token)
    }
}

fn parse_math_expression(tokens: &mut Vec<Token>) -> Expression {
    let mut nodes: Vec<Expression> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();
    while tokens.len() > 0 {
        match tokens[0].clone() {
            Token::Value(value) => {
                nodes.push(Expression::Value(value));
                tokens.remove(0);
            },
            Token::OpenParenthesis => {
                stack.push(tokens[0].clone());
                tokens.remove(0);
            },
            Token::CloseParenthesis => {
                loop {
                    match stack.pop() {
                        Some(token) => {
                            match token {
                                Token::OpenParenthesis => break,
                                _ => {
                                    let right = nodes.pop().unwrap();
                                    let left = nodes.pop().unwrap();
                                    nodes.push(build_node(token, left, right));
                                },
                            }
                        },
                        None => panic!("Can not find open parenthesis"),
                    };
                }
                tokens.remove(0);
            },
            _ => {
                match get_priority(tokens[0].clone()) {
                    Some(priority) => {
                        while stack.len() > 0 && get_priority(stack.last().unwrap().clone()).unwrap() >= priority {
                            let right = nodes.pop().unwrap();
                            let left = nodes.pop().unwrap();
                            nodes.push(build_node(stack.pop().unwrap(), left, right));
                        }
                        stack.push(tokens[0].clone());
                        tokens.remove(0);
                    },
                    None => break,
                }
            },
        };
    }
    while stack.len() > 0 {
        let right = nodes.pop().unwrap();
        let left = nodes.pop().unwrap();
        nodes.push(build_node(stack.pop().unwrap(), left, right));
    }
    return nodes[0].clone();
}

fn write_tree(node: &Expression, file: &mut File) {
    match node {
        Expression::If(condition, true_branch, false_branch) => {
            file.write_all(b"condition(").unwrap();
            write_tree(condition.as_ref(), file);
            file.write_all(b")-true_branch(").unwrap();
            write_tree(true_branch.as_ref(), file);
            if let Some(false_branch) = false_branch {
                file.write_all(b")-false_branch(").unwrap();
                write_tree(false_branch.as_ref(), file);
            }
            file.write_all(b")").unwrap();
        },
        Expression::BinaryMathOperation(operator, left, right) => {
            write_tree(left.as_ref(), file);
            write_tree(right.as_ref(), file);
            file.write_all(format!("{} ", operator).as_bytes()).unwrap();
        },
        Expression::Value(value) => file.write_all(format!("{} ", value).as_bytes()).unwrap(),
    }
}

fn parse_expression(tokens: &mut Vec<Token>) -> Expression {
    match tokens[0].clone() {
        Token::If => {
            tokens.remove(0);
            let condition = parse_math_expression(tokens);
            tokens.remove(0);
            let true_branch = parse_math_expression(tokens);
            let mut false_branch = Option::None;
            match tokens[0].clone() {
                Token::Else => {
                    tokens.remove(0);
                    false_branch = Option::Some(Box::new(parse_math_expression(tokens)));
                },
                _ => {},
            }
            tokens.remove(0);
            Expression::If(Box::new(condition), Box::new(true_branch), false_branch)
        },
        _ => parse_math_expression(tokens),
    }
}

pub fn convert(tokens: &mut Vec<Token>) {
    let root_node = parse_expression(tokens);
    println!("{:?}", &root_node);
    let mut postfix = File::create("result").unwrap();
    write_tree(&root_node, &mut postfix);
}
