// This function parses an assignment expression from the given tokens.

use std::io::ErrorKind;

use crate::{ expressions::expressions::Expression::{ self, * }, tokens::tokens::Token::{ self } };

use super::main_parser::{ parse_infix, parse_prefix };

/// Parses an assignment expression.
pub fn parse_assignment(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    // Parse the right-hand side of the assignment
    let (right, tokens_after) = parse_prefix(1, tokens[1..].to_vec())?;

    // Combine left and right expressions into the assignment expression
    parse_infix(
        AssignExp {
            left: Box::new(left),
            right: Box::new(right),
        },
        tokens_after,
        prec_limit
    )
}
