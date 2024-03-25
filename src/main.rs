use std::collections::HashMap;

use crate::{
    evaluator::evaluator::eval_prog,
    lexer::tokeniser::tokenise,
    parsers::main_parser::{ parse, parse_programme },
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
        eval_prog(
            parse_programme(tokenise("date = 06//11//23; date = date + 3 * h; date")),
            &HashMap::new()
        )
    );
}
