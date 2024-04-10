/// Enum representing various types of tokens in the language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Operator(String),
    TimeLength(Time),
    Date(String),
    LParen(String),
    RParen(String),
    Var(String),
    Semi,
}

/// Enum representing different units of time for the `TimeLength` token.
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
