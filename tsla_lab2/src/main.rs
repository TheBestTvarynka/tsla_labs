use std::io::{stdin, stdout, Write};
use tsla_lab2::converter::convert;
use tsla_lab2::lexer::tokenize;

fn main() {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("Readed: {}", s);
    convert(&mut tokenize(&mut s));
}
