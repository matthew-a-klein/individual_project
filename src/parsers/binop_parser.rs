// This function parses a binary operation expression from the given tokens.

use std::io::ErrorKind;

use crate::{
    expressions::expressions::Expression::{ self, * },
    tokens::{ token_classification::get_precedence, tokens::Token::{ self, * } },
};

use super::main_parser::{ parse_infix, parse_prefix };

/// Parses a binary operation expression.
pub fn parse_binop(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    // Ensure the first token is an operator
    if let Operator(s) = &tokens[0] {
        let limit = get_precedence(&tokens[0]);
        // Parse the right-hand side expression
        let (right, tokens_after) = parse_prefix(limit, tokens[1..].to_vec())?;
        // Combine left, operator, and right expressions into the binary operation expression
        parse_infix(
            InfixExp {
                left: Box::new(left),
                op: s.to_string(),
                right: Box::new(right),
            },
            tokens_after,
            prec_limit
        )
    } else {
        // Panic if the first token is not an operator
        panic!("Unexpected token, expected operator")
    }
}
