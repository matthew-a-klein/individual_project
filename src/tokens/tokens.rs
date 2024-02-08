#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Operator(String),

    TimeLength(Time),
    InfixOp {
        left: Vec<Token>,
        op: Box<Token>,
        right: Vec<Token>,
    },
    PostfixOp {
        operand: Vec<Token>,
        op: Box<Token>,
    },
    PrefixOp {
        op: Box<Token>,
        right: Vec<Token>,
    },
    Date(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Time {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Time,
    Date,
}
#[derive(Debug, Clone, PartialEq)]
pub struct TypedToken {
    pub tok: Token,
    pub ty: Type,
}
