use crate::{ expressions::expressions::Expression::{ self, * }, tokens::tokens::Token::{ self } };

use super::main_parser::parse_prefix;

pub fn parse_assignment(
    left: Expression,
    tokens: Vec<Token>,
    _prec_limit: i32
) -> (Expression, Vec<Token>) {
    if let VarExp(s) = left {
        let (right, tokens_after) = parse_prefix(1, tokens[1..].to_vec());

        (
            AssignExp {
                name: s,
                right: Box::new(right),
            },
            tokens_after,
        )
    } else {
        panic!()
    }
}
