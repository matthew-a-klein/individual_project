#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Operator(String),
    TimeLength(Time),
    Date(String),
    LParen(String),
    RParen(String),
    Var(String),
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
