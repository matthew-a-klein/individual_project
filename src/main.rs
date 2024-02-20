use std::collections::HashMap;

use crate::{
    evaluator::evaluator::eval_stmt, lexer::tokeniser::tokenise, parsers::main_parser::parse_all,
};
mod evaluator;
mod expressions;
mod lexer;
mod parsers;
mod regex;
mod tokens;
fn main() -> () {
    println!(
        "{:?}",
        eval_stmt(parse_all(tokenise("date = 11//12//23")), &HashMap::new())
    )
}
