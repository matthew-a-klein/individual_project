use std::io::ErrorKind;

use crate::{
    expressions::expressions::Expression::{ self, * },
    tokens::tokens::Token::{ self, * },
};

use super::main_parser::parse_infix;

pub fn parse_number(
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    if let Number(n) = tokens[0] {
        parse_infix(NumberExp(n), tokens[1..].to_vec(), prec_limit)
    } else {
        panic!()
    }
}
