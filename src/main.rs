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
                tokenise(
                    "
            low = 1; 
            high = 1;
            truth = low == high ? 1 : 0 + 2;
            truth
            "
                )
            )
        )
    );
}
