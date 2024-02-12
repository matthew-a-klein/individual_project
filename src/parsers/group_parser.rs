use crate::{
    expressions::expressions::Expression::{self},
    tokens::tokens::Token::{self, *},
};

use super::main_parser::{parse_infix, parse_prefix};

pub fn parse_group(tokens: Vec<Token>, prec_limit: i32) -> (Expression, Vec<Token>) {
    println!("{:?}", tokens);
    let l_paren = "(".to_string();
    let r_paren = ")".to_string();
    if let LParen(_l_paren) = &tokens[0] {
        let (exp, tokens_after) = parse_prefix(0, tokens[1..].to_vec());
        if let RParen(_r_paren) = &tokens_after[0] {
            parse_infix(exp, tokens_after[1..].to_vec(), prec_limit)
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}
