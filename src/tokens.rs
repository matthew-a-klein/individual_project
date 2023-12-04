#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Operator(Operator),
    LParen,
    RParen,
    Binop {
        left: Vec<Token>,
        operator: Operator,
        right: Vec<Token>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    ADD,
    SUB,
    DIVIDE,
    MULTIPLY,
    POWER,
}
