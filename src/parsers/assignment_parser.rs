use crate::{ expressions::expressions::Expression::{ self, * }, tokens::tokens::Token::{ self } };

use super::main_parser::{ parse_infix, parse_prefix };

pub fn parse_assignment(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> (Expression, Vec<Token>) {
    let (right, tokens_after) = parse_prefix(1, tokens[1..].to_vec());

    parse_infix(
        AssignExp {
            left: Box::new(left),
            right: Box::new(right),
        },
        tokens_after,
        prec_limit
    )
}
