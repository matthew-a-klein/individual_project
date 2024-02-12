use crate::{
    expressions::expressions::Expression::{self},
    tokens::tokens::Token::{self},
};

pub fn parse_r_paren(
    exp: Expression,
    tokens: Vec<Token>,
    prec_limit: i32,
) -> (Expression, Vec<Token>) {
    (exp, tokens)
}
