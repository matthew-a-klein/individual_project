use std::collections::HashMap;

use crate::{
    evaluator::evaluator::evaluate,
    lexer::tokeniser::tokenise,
    parsers::main_parser::{ parse_programme },
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
        evaluate(
            parse_programme(
                tokenise("
            number = 2* ((3 + 1)* (2- 1));
            number
            ")
            )
        )
    );
}
