#[derive(Debug)]
pub enum TokenType {
    Let,
    Mut,
    Name,
    Fn,
    OpenP,
    CloseP,
    OpenCurly,
    CloseCurly,
    Assign,
    Equal,
    Semicolon,
    Colon,
    RightArrow,
    U32Literal,
    F32Literal,
    StringLiteral,
    I42Literal,
    Relop,
    Type,
    MacrosSymbol,
    ArithmeticOperation,
}

#[derive(Debug)]
pub enum Value {
    Value(String),
    Ref(u64),
}

pub struct Token {
    pub token_type: TokenType,
    pub value: Value,
    pub line: u64,
    pub position: u64,
}

impl Token {
    pub const fn new(token_type: TokenType, value: Value, line: u64, position: u64) -> Self {
        Token {
            token_type, value, line, position
        }
    }
}
