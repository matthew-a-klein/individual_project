use std::io::ErrorKind;

use crate::{
    expressions::expressions::Expression::{ self, ConditionalExp },
    tokens::tokens::Token,
};

use super::main_parser::{ parse, parse_infix, parse_prefix };

pub fn parse_conditional(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    let (then_arm, tokens_remaining) = parse(tokens[1..].to_vec())?;
    match &tokens_remaining[0] {
        Token::Operator(s) if s.as_str() == ":" => {
            let (else_arm, tokens_still_remaining) = parse_prefix(
                1,
                tokens_remaining[1..].to_vec()
            )?;
            parse_infix(
                ConditionalExp {
                    condition: Box::new(left),
                    if_branch: Box::new(then_arm),
                    else_branch: Box::new(else_arm),
                },
                tokens_still_remaining,
                prec_limit
            )
        }
        _ => Err(ErrorKind::InvalidInput),
    }
}
