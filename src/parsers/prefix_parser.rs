use std::io::ErrorKind;

use crate::{
    expressions::expressions::Expression::{ self, * },
    tokens::{ token_classification::get_precedence, tokens::Token::{ self, * } },
};

use super::main_parser::{ parse_infix, parse_prefix };

//Parses a prefix expression
pub fn parse_prefix_op(
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    if let Operator(s) = &tokens[0] {
        let limit = get_precedence(&tokens[0]);
        let (right, tokens_after) = parse_prefix(limit, tokens[1..].to_vec())?;
        parse_infix(
            PrefixExp {
                op: s.to_string(),
                right: Box::new(right),
            },
            tokens_after,
            prec_limit
        )
    } else {
        panic!("Unexpected token, expected operator.")
    }
}
