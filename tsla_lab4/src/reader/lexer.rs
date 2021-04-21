use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::reader::token::{Token, TokenType, Value};
use std::collections::HashMap;
use regex::Regex;

pub struct GeneratedTable {
    tokens: Vec<Token>,
    table: HashMap<u64, String>,
}

impl GeneratedTable {
    pub fn new() -> Self {
        GeneratedTable {
            tokens: Vec::new(),
            table: HashMap::new(),
        }
    }

    pub fn print(&self) {
        println!("Table:");
        for (id, value) in &self.table {
            println!("{} -> {}", id, value);
        }
        println!("Tokens:");
        for token in &self.tokens {
            println!("{:?} {:?} {} {}", &token.token_type, &token.value, token.line, token.position);
        }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn add_value(&mut self, value: String) -> u64 {
        let size = self.table.len() as u64;
        self.table.insert(size, value);
        size
    }
}

fn get_token(line: String, table: &mut GeneratedTable, line_numer: u64, position: u64) -> (String, u64) {
    let re_int = Regex::new(r"^\d+").unwrap();
    let re_float = Regex::new(r"^[0-9]*\.[0-9]*").unwrap();
    let re_name = Regex::new(r"^[a-zA-Z][a-zA-Z_0-9]*").unwrap();
    let re_arithmetic = Regex::new(r"^(\+|-|\*)").unwrap();
    let re_relop = Regex::new(r"^(==)|(!=)|>|<").unwrap();

    if line.starts_with("(") {
        table.add_token(Token::new(TokenType::OpenP, Value::Value("(".to_owned()), line_numer, position));
        (line[1..].to_string(), 1)
    } else if line.starts_with(")") {
        table.add_token(Token::new(TokenType::CloseP, Value::Value(")".to_owned()), line_numer, position));
        (line[1..].to_string(), 1)
    } else if line.starts_with("{") {
        table.add_token(Token::new(TokenType::OpenCurly, Value::Value("{".to_owned()), line_numer, position));
        (line[1..].to_string(), 1)
    } else if line.starts_with("}") {
        table.add_token(Token::new(TokenType::CloseCurly, Value::Value("}".to_owned()), line_numer, position));
        (line[1..].to_string(), 1)
    } else if line.starts_with("let") {
        table.add_token(Token::new(TokenType::Let, Value::Value("let".to_owned()), line_numer, position));
        (line[3..].to_string(), 3)
    } else if line.starts_with("mut") {
        table.add_token(Token::new(TokenType::Mut, Value::Value("mut".to_owned()), line_numer, position));
        (line[3..].to_string(), 3)
    } else if line.starts_with("fn") {
        table.add_token(Token::new(TokenType::Fn, Value::Value("fn".to_owned()), line_numer, position));
        (line[2..].to_string(), 2)
    } else if line.starts_with(";") {
        table.add_token(Token::new(TokenType::Semicolon, Value::Value(";".to_owned()), line_numer, position));
        (line[1..].to_string(), 1)
    } else if line.starts_with(":") {
        table.add_token(Token::new(TokenType::Colon, Value::Value(":".to_owned()), line_numer, position));
        (line[1..].to_string(), 1)
    } else if line.starts_with("=") {
        table.add_token(Token::new(TokenType::Assign, Value::Value("=".to_owned()), line_numer, position));
        (line[1..].to_string(), 1)
    } else if line.starts_with("\n") {
        (line[1..].to_string(), 1)
    } else if line.starts_with("u32") {
        table.add_token(Token::new(TokenType::Type, Value::Value("u32".to_owned()), line_numer, position));
        (line[3..].to_string(), 3)
    } else if line.starts_with("f32") {
        table.add_token(Token::new(TokenType::Type, Value::Value("f32".to_owned()), line_numer, position));
        (line[3..].to_string(), 3)
    } else if let Some(res) = re_float.find(&line) {
        println!("res: int: {:?}", res);
        let value = line[res.start()..res.end()].to_string();
        let id = table.add_value(value);
        table.add_token(Token::new(TokenType::F32Literal, Value::Ref(id), line_numer, position));
        (line[res.end()..].to_string(), (res.end() - res.start()) as u64)
    } else if let Some(res) = re_arithmetic.find(&line) {
        println!("res: int: {:?}", res);
        let value = line[res.start()..res.end()].to_string();
        let id = table.add_value(value);
        table.add_token(Token::new(TokenType::ArithmeticOperation, Value::Ref(id), line_numer, position));
        (line[res.end()..].to_string(), (res.end() - res.start()) as u64)
    } else if let Some(res) = re_relop.find(&line) {
        println!("res: int: {:?}", res);
        let value = line[res.start()..res.end()].to_string();
        let id = table.add_value(value);
        table.add_token(Token::new(TokenType::Relop, Value::Ref(id), line_numer, position));
        (line[res.end()..].to_string(), (res.end() - res.start()) as u64)
    } else if let Some(res) = re_int.find(&line) {
        println!("res: int: {:?}", res);
        let value = line[res.start()..res.end()].to_string();
        let id = table.add_value(value);
        table.add_token(Token::new(TokenType::U32Literal, Value::Ref(id), line_numer, position));
        (line[res.end()..].to_string(), (res.end() - res.start()) as u64)
    } else if let Some(res) = re_name.find(&line) {
        println!("res: name: {:?}", res);
        let value = line[res.start()..res.end()].to_string();
        let id = table.add_value(value);
        table.add_token(Token::new(TokenType::Name, Value::Ref(id), line_numer, position));
        (line[res.end()..].to_string(), (res.end() - res.start()) as u64)
    } else {
        // println!("skip: {:?}", line[0..1].to_string());
        (line[1..].to_string(), 1)
    }
}

fn parse_line(mut line: String, table: &mut GeneratedTable, line_number: u64) {
    let mut position = 1;
    while line.len() > 0 {
        let (new_line, add_pos) = get_token(line, table, line_number, position);
        position += add_pos;
        line = new_line;
    }
}

pub fn parse_file(filename: &str) -> Result<GeneratedTable, String> {
    let code = BufReader::new(File::open(filename).map_err(|err| format!("Error with file opening: {:?}", err))?);    
    let mut table = GeneratedTable::new();
    let mut line_number = 1;
    for line in code.lines() {
        let line = line.map_err(|err| format!("Cannot read line: {:?}", err))?;
        println!("{:?}", &line);
        parse_line(line, &mut table, line_number);
        line_number += 1;
    }
    Result::Ok(table)
}