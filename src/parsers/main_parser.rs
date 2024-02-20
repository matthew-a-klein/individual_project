use crate::{
    expressions::expressions::Expression,
    parsers::{
        assignment_parser::parse_assignment, date_parser::parse_date, number_parser::parse_number,
        prefix_parser::parse_prefix_op, r_paren_parser::parse_r_paren, time_parser::parse_time,
    },
    tokens::tokens::Token,
};

use super::{binop_parser::parse_binop, group_parser::parse_group, var_parser::parse_var};

pub fn parse_prefix(limit: i32, tokens: Vec<Token>) -> (Expression, Vec<Token>) {
    if tokens.len() == 0 {
        (Expression::Empty, Vec::new())
    } else {
        match &tokens[0] {
            Token::Number(_) => parse_number(tokens, limit),
            Token::Operator(_) => parse_prefix_op(tokens, limit),
            Token::TimeLength(_) => parse_time(tokens, limit),
            Token::Date(_) => parse_date(tokens, limit),
            Token::LParen(_) => parse_group(tokens, limit),
            Token::Var(_) => parse_var(tokens, limit),
            _ => panic!(),
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
            Token::Operator(s) if s == "=" => parse_assignment(left, tokens, prec_limit),
            Token::Operator(_) => parse_binop(left, tokens, prec_limit),
            Token::RParen(_) => parse_r_paren(left, tokens, prec_limit),
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
