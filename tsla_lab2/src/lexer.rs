use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    If,
    Then,
    Else,
    End,
    EndIf,
    Plus,
    Minus,
    Devide,
    Multiply,
    OpenParenthesis,
    CloseParenthesis,
    Less,
    Greater,
    Equal,
    NotEqual,
    Value(u32)
}

fn next_token(mut code: &mut str) -> Option<(Token, &mut str)> {
    if code.len() == 0 {
        return Option::None;
    }
    let re_number = Regex::new(r"\d+").unwrap();
    let clone = code.to_owned();
    loop {
        if code.starts_with("if") {
            return Option::Some((Token::If, &mut code[2..]));
        } else if code.starts_with("else") {
            return Option::Some((Token::Else, &mut code[4..]));
        } else if code.starts_with("then") {
            return Option::Some((Token::Then, &mut code[4..]));
        } else if code.starts_with("end") {
            return Option::Some((Token::Then, &mut code[3..]));
        } else if code.starts_with("+") {
            return Option::Some((Token::Plus, &mut code[1..]));
        } else if code.starts_with("-") {
            return Option::Some((Token::Minus, &mut code[1..]));
        } else if code.starts_with("*") {
            return Option::Some((Token::Multiply, &mut code[1..]));
        } else if code.starts_with("/") {
            return Option::Some((Token::Devide, &mut code[1..]));
        } else if code.starts_with("<") {
            return Option::Some((Token::Less, &mut code[1..]));
        } else if code.starts_with(">") {
            return Option::Some((Token::Greater, &mut code[1..]));
        } else if code.starts_with("==") {
            return Option::Some((Token::Equal, &mut code[2..]));
        } else if code.starts_with("!+") {
            return Option::Some((Token::NotEqual, &mut code[2..]));
        } else if code.starts_with("(") {
            return Option::Some((Token::OpenParenthesis, &mut code[1..]));
        } else if code.starts_with(")") {
            return Option::Some((Token::CloseParenthesis, &mut code[1..]));
        } else if let Some(res) = re_number.find(&clone) {
            if let Ok(number) = code[res.start()..res.end()].parse::<u32>() {
                return Option::Some((Token::Value(number), &mut code[res.end()..]));
            } else {
                panic!("Invalid number");
            }
        } else if code.starts_with(" ") {
            code = &mut code[1..];
            continue;
        } else if code.len() == 0 {
            return Option::None;
        } else {
            panic!("Unexpected token");
        }
    }
}

pub fn tokenize(mut code: &mut str) -> Vec<Token> {
    let mut tokens = Vec::new();
    while let Some((token, rest_code)) = next_token(code) {
        tokens.push(token);
        code = rest_code;
    }
    return tokens;
}