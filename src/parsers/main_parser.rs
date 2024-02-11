use crate::{
    expressions::expressions::Expression,
    parsers::{
        date_parser::parse_date, number_parser::parse_number, prefix_parser::parse_prefix_op,
        time_parser::parse_time,
    },
    tokens::tokens::Token,
};

use super::binop_parser::parse_binop;

pub fn parse_prefix(limit: i32, tokens: Vec<Token>) -> (Expression, Vec<Token>) {
    if tokens.len() == 0 {
        (Expression::Empty, Vec::new())
    } else {
        match &tokens[0] {
            Token::Number(_) => parse_number(tokens, limit),
            Token::Operator(_) => parse_prefix_op(tokens, limit),
            Token::TimeLength(_) => parse_time(tokens, limit),
            Token::Date(_) => parse_date(tokens, limit),
        }
    }
}

pub fn parse_infix(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32,
) -> (Expression, Vec<Token>) {
    if tokens.len() == 0 {
        (left, Vec::new())
    } else {
        match &tokens[0] {
            Token::Operator(_) => parse_binop(left, tokens, prec_limit),
            _ => panic!(),
        }
    }
}

pub fn parse(toks: Vec<Token>) -> (Expression, Vec<Token>) {
    parse_prefix(0, toks)
}

pub fn parse_all(toks: Vec<Token>) -> Expression {
    let (parsed, unparsed) = parse(toks);
    if unparsed.is_empty() {
        parsed
    } else {
        panic!("Your programme could not be parsed")
    }
}
