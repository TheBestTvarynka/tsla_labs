use crate::reader::lexer::GeneratedTable;

// Enum with all supported token types
#[derive(Debug, Clone, Copy)]
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
    Empty
}

// enum with token value. token value can be one of the two operions:
// 1) value - data that stored inside the token
// 2) ref (stands for references) - id to the value in the table
#[derive(Debug, Clone)]
pub enum Value {
    Value(String),
    Ref(u64),
}

// token structure
// token holds following data: type, value, line of this token, position at this line
#[derive(Clone, Debug)]
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

    pub fn empty() -> Self {
        Token {
            token_type: TokenType::Empty,
            value: Value::Value("empty token".to_owned()),
            line: 0,
            position: 0,
        }
    }

    pub fn get_value(&self, table: &GeneratedTable) -> String {
        match self.value.clone() {
            Value::Value(value) => value,
            Value::Ref(id) => table.get_value(id),
        }
    }
}
