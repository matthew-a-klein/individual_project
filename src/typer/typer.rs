use crate::tokens::tokens::{Token, Type, Type::*};

pub fn type_tok(tok: &Token) -> Type {
    match &tok {
        Token::TimeLength(_) => Time,
        Token::InfixOp { left, op, right } => match (type_tok(&left[0]), type_tok(&right[0])) {
            (Time, Time) => Time,
            (Time, Date) => unreachable!(),
            (Date, Time) => Date,
            (Date, Date) => Time,
        },
        Token::PostfixOp { operand, op } => type_tok(&op),
        Token::Date(_) => Date,
        _ => unreachable!(),
    }
}
