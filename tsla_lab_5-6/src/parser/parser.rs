use crate::parser::tree::{Node};
use crate::parser::grammar::LRTable;
use crate::reader::token::Token;
use crate::reader::lexer::GeneratedTable;

#[derive(Debug)]
pub struct Stack {
    states: Vec<Node>,
}

impl Stack {
    pub const fn new() -> Self {
        Stack {
            states: Vec::new(),
        }
    }

    pub fn add_state(&mut self, node: Node) {
        self.states.push(node);
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn get_nth_name(&self, index: usize) -> String {
        self.states[self.states.len() - 1 - index].name.clone()
    }

    pub fn pop(&mut self) -> Node {
        self.states.pop().unwrap()
    }

    pub fn print_names(&self) {
        for i in &self.states {
            print!("{} ", &i.name);
        }
        println!("");
    }
}

fn parse(tokens: &Vec<Token>, table: &GeneratedTable) -> Result<Node, String> {
    let mut stack = Stack::new();
    let mut tokens = tokens.into_iter();
    let lr_tree = LRTable::load_default();
    if let Some(token) = tokens.next() {
        stack.add_state(Node::from_token(token.clone(), table));
    }
    while let Some(token) = tokens.next() {
        let new_node = Node::from_token(token.clone(), table);
        lr_tree.perform_action(&mut stack, new_node);
    }
    while stack.len() > 1 {
        stack.print_names();
        lr_tree.force_reduct(&mut stack);
    }
    stack.print_names();
    Result::Ok(stack.pop())
}

pub fn build_tree(table: GeneratedTable) -> Result<Node, String> {
    parse(&table.tokens, &table)
}
