use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};
use crate::parser::tree::{NodeType, Node};
use crate::parser::parser::Stack;
use crate::reader::token::{Token};

#[derive(Clone)]
pub struct Types {
    pub types: Vec<String>,
}

impl Types {
    pub fn from_vec(types: Vec<String>) -> Self {
        Types {
            types,
        }
    }

    pub fn starts_with(&self, pattern: &Vec<String>) -> bool {
        // if pattern.len() >= self.types.len() {
        if pattern.len() > self.types.len() {
            false
        } else {
            for i in 0..pattern.len() {
                if self.types[i] != pattern[i] {
                    return false;
                }
            }
            true
        }
    }

    pub fn equal(&self, pattern: &Vec<String>) -> bool {
        if self.types.len() != pattern.len() {
            false
        } else {
            for i in 0..pattern.len() {
                if pattern[i] != self.types[i] {
                    return false;
                }
            }
            true
        }
    }
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        if self.types.len() != other.types.len() {
            return false;
        }
        for i in 0..self.types.len() {
            if self.types[i] != other.types[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Types { }

impl Hash for Types {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for t in &self.types {
            t.hash(state);
        }
    }
}

type ReductionFn = Box<dyn Fn(Vec<Box<Node>>) -> Node>;

pub struct LRTable {
    rules: HashMap<Types, NodeType>,
    reduction_fns: HashMap<Types, ReductionFn>,
    non_terminals: Vec<String>,
    max_len: usize,
}

pub enum ActionType {
    Move(Node),
    Reduction(u32),
    Error(String),
    Finish(Node)
}

impl LRTable {
    pub fn empty() -> Self {
        LRTable {
            rules: HashMap::new(),
            reduction_fns: HashMap::new(),
            non_terminals: Vec::new(),
            max_len: 0,
        }
    }

    pub fn add_rule(&mut self, from: Vec<String>, to: NodeType) {
        if from.len() > self.max_len {
            self.max_len = from.len();
        }
        self.rules.insert(Types::from_vec(from), to);
    }

    pub fn add_reduction_fn(&mut self, from: Vec<String>, to: ReductionFn) {
        self.reduction_fns.insert(Types::from_vec(from), to);
    }

    pub fn load_default() -> Self {
        let mut table = LRTable::empty();
        table.non_terminals = vec![
            "Lit".to_owned(),
            "T".to_owned(),
            "P".to_owned(),
            "Expression".to_owned()
            // ,"let".to_owned(),
        ];

        table.add_rule(vec!["Name".to_owned()], NodeType::Lit);
        table.add_rule(vec!["Lit".to_owned()], NodeType::T);
        table.add_rule(vec!["T".to_owned(), "*".to_owned(), "T".to_owned()], NodeType::T);
        table.add_rule(vec!["T".to_owned(), "/".to_owned(), "T".to_owned()], NodeType::T);
        table.add_rule(vec!["T".to_owned()], NodeType::P);
        table.add_rule(vec!["P".to_owned(), "+".to_owned(), "P".to_owned()], NodeType::P);
        table.add_rule(vec!["P".to_owned(), "-".to_owned(), "P".to_owned()], NodeType::P);
        table.add_rule(vec!["P".to_owned()], NodeType::Expression);
        table.add_rule(vec!["Expression".to_owned(), "<".to_owned(), "Expression".to_owned()], NodeType::Expression);
        table.add_rule(vec!["Expression".to_owned(), ">".to_owned(), "Expression".to_owned()], NodeType::Expression);
        table.add_rule(vec!["Expression".to_owned(), "!=".to_owned(), "Expression".to_owned()], NodeType::Expression);
        table.add_rule(vec!["Expression".to_owned(), "==".to_owned(), "Expression".to_owned()], NodeType::Expression);
        table.add_rule(vec![
            "let".to_owned(),
            "Name".to_owned(),
            "=".to_owned(),
            "Expression".to_owned(),
            ";".to_owned()
        ], NodeType::Statement);
        table.add_rule(vec!["Statement".to_owned()], NodeType::Program);
        table.add_rule(vec!["Program".to_owned(), "Program".to_owned()], NodeType::Program);
        table.add_rule(vec!["Program".to_owned(), "let".to_owned()], NodeType::Program);

        let binary_fn = |node_type: NodeType| -> ReductionFn {
            Box::new(move |mut childs| {
                let node = childs.remove(1);
                Node {
                    name: NodeType::to_string(node_type),
                    childs,
                    node_type,
                    params: HashMap::new(),
                    token: node.token,
                }
            })
        };
        let unary_fn = |node_type: NodeType| -> ReductionFn {
            Box::new(move |childs| Node {
                name: NodeType::to_string(node_type),
                childs,
                node_type,
                params: HashMap::new(),
                token: Token::empty(),
            })
        };

        table.add_reduction_fn(vec!["Lit".to_owned()], unary_fn(NodeType::T));
        table.add_reduction_fn(vec!["T".to_owned(), "*".to_owned(), "T".to_owned()], binary_fn(NodeType::T));
        table.add_reduction_fn(vec!["T".to_owned(), "/".to_owned(), "T".to_owned()], binary_fn(NodeType::T));
        table.add_reduction_fn(vec!["T".to_owned()], unary_fn(NodeType::P));
        table.add_reduction_fn(vec!["P".to_owned(), "+".to_owned(), "P".to_owned()], binary_fn(NodeType::P));
        table.add_reduction_fn(vec!["P".to_owned(), "-".to_owned(), "P".to_owned()], binary_fn(NodeType::P));
        table.add_reduction_fn(vec!["P".to_owned()], unary_fn(NodeType::Expression));
        table.add_reduction_fn(vec!["Expression".to_owned(), ">".to_owned(), "Expression".to_owned()], binary_fn(NodeType::Expression));
        table.add_reduction_fn(vec!["Expression".to_owned(), "<".to_owned(), "Expression".to_owned()], binary_fn(NodeType::Expression));
        table.add_reduction_fn(vec!["Expression".to_owned(), "==".to_owned(), "Expression".to_owned()], binary_fn(NodeType::Expression));
        table.add_reduction_fn(vec!["Expression".to_owned(), "!=".to_owned(), "Expression".to_owned()], binary_fn(NodeType::Expression));
        table.add_reduction_fn(vec![
            "let".to_owned(),
            "Name".to_owned(),
            "=".to_owned(),
            "Expression".to_owned(),
            ";".to_owned()
        ], Box::new(|childs| Node {
            name: "Statement".to_owned(),
            node_type: NodeType::Statement,
            params: HashMap::new(),
            childs: vec![childs[1].clone(), childs[3].clone()],
            token: Token::empty(),
        }));
        table.add_reduction_fn(vec!["Statement".to_owned()], unary_fn(NodeType::Program));
        table.add_reduction_fn(vec!["Program".to_owned(), "Statement".to_owned()], unary_fn(NodeType::Program));
        table.add_reduction_fn(vec!["Program".to_owned(), "Program".to_owned()], unary_fn(NodeType::Program));
        table
    }

    pub fn get_reduct_fn(&self, from: Vec<String>) -> &ReductionFn {
        self.reduction_fns.get(&Types::from_vec(from)).unwrap()
    }

    fn if_contain_rule(&self, pattern: &Vec<String>) -> bool {
        println!("if_contain_rule: {:?}", pattern);
        let types = Types::from_vec(pattern.clone());
        let res = self.rules.contains_key(&types);
        println!("res: {}", res);
        res
    }

    fn if_contain_similar_rule(&self, pattern: &Vec<String>) -> bool {
        println!("if_contain_similar: {:?}", pattern);
        for (key, _) in self.rules.clone() {
            if key.starts_with(pattern) {
                println!("true");
                return true;
            }
        }
        println!("false");
        false
    }

    fn if_non_terminal(&self, symbol: &String) -> bool {
        println!("if_terminal: {}", symbol);
        for i in &self.non_terminals {
            if i == symbol {
                println!("res: true");
                return true
            }
        }
        println!("res: false");
        false
    }

    fn count_similar_rules(&self, pattern: &Vec<String>) -> usize {
        println!("count similar rules: {:?}", pattern);
        let mut count = 0;
        for (key, _) in self.rules.clone() {
            if key.starts_with(pattern) {
                count += 1;
            }
        }
        println!("res: {}", count);
        count
    }

    fn reduct(&self, stack: &mut Stack, reduction_size: usize) {
        let mut reduct_pattern = Vec::new();
        let mut reduct_nodes = Vec::new();
        for _ in 0..reduction_size {
            reduct_pattern.insert(0, stack.get_nth_name(0));
            reduct_nodes.insert(0, Box::new(stack.pop()));
        }
        println!("reduction of {:?}", reduct_pattern);
        stack.add_state(self.get_reduct_fn(reduct_pattern)(reduct_nodes));
    }

    pub fn force_reduct(&self, stack: &mut Stack) {
        let mut reduction_size = 0;
        let max = if stack.len() >= self.max_len { self.max_len } else { stack.len() };
        let mut pattern = Vec::new();
        for i in 0..max {
            pattern.insert(0, stack.get_nth_name(i));
            if self.if_contain_rule(&pattern) {
                reduction_size = i;
            }
        }
        println!("lens: {} {}", stack.len(), reduction_size);
        self.reduct(stack, reduction_size + 1);
    }

    pub fn perform_action(&self, stack: &mut Stack, node: Node) {
        loop {
            println!("stack:");
            stack.print_names();
            println!("node: {:?}", node);
            let max = if stack.len() >= self.max_len { self.max_len } else { stack.len() };
            println!("max: {}", max);
            wait();
            let mut pattern1 = Vec::new();
            let mut pattern2 = vec![node.name.clone()];
            let mut reduction_size = -1;
            for i in 0..max {
                pattern1.insert(0, stack.get_nth_name(i));
                pattern2.insert(0, stack.get_nth_name(i));
                // println!("cur pattern: {:?}", pattern1);
                if self.if_contain_similar_rule(&pattern2) {
                    println!("add to state: {:?}", node);
                    stack.add_state(node);
                    println!("return");
                    return;
                }
                if self.if_contain_similar_rule(&pattern1) && self.if_non_terminal(&node.name) {
                    println!("add to state: {:?}", node);
                    stack.add_state(node);
                    println!("return");
                    return;
                }
                if self.if_contain_rule(&pattern1) {
                    reduction_size = i as i32;
                    // break;
                }
            }
            if reduction_size >= 0 {
                println!("perform reduction:");
                self.reduct(stack, reduction_size as usize + 1);
            }
        }
    }
}

fn wait() {
    use std::io::{stdin, stdout, Write};
    print!("enter smth: ");
    let mut s = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("all good");
}