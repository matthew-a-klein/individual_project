use crate::{
    expressions::expressions::Expression::{self, *},
    tokens::{
        token_classification::get_precedence,
        tokens::Token::{self, *},
    },
};

use super::main_parser::{parse_infix, parse_prefix};

pub fn parse_binop(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32,
) -> (Expression, Vec<Token>) {
    if let Operator(s) = &tokens[0] {
        let limit = get_precedence(&tokens[0]);

        if limit <= prec_limit {
            (left, tokens)
        } else {
            let (right, tokens_after) = parse_prefix(limit, tokens[1..].to_vec());
            parse_infix(
                InfixExp {
                    left: Box::new(left),
                    op: s.to_string(),
                    right: Box::new(right),
                },
                tokens_after,
                prec_limit,
            )
        }
    } else {
        panic!()
    }
}
