use std::io::{stdin,stdout,Write};
use regex::Regex;

fn check_word(data: String) -> bool {
    Regex::new(r"(a|b)\+(a|b)\-(a|b)")
        .unwrap()
        .is_match(data.as_str())
}

fn main() {
    println!("Enter a word:");
    let mut s = String::new();
    let _=stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    println!("Result: {}", check_word(s));
}
