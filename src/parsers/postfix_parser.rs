use std::io::ErrorKind;

use crate::{
    expressions::expressions::Expression::{ self, * },
    tokens::tokens::Token::{ self, * },
};

use super::main_parser::parse_infix;

// Parses a postfix expression.
pub fn parse_postfix(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    if let Operator(s) = &tokens[0] {
        parse_infix(
            PostfixExp {
                left: Box::new(left),
                op: s.to_string(),
            },
            tokens[1..].to_vec(),
            prec_limit
        )
    } else {
        Err(ErrorKind::InvalidInput)
    }
}
