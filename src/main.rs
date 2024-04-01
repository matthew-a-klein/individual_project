use std::collections::HashMap;

use crate::{
    evaluator::evaluator::evaluate,
    lexer::tokeniser::tokenise,
    parsers::main_parser::{ parse, parse_all, parse_programme },
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

        parse_programme(
            tokenise(
                "functioncall(argone, argtwo) = argone == argtwo? argone: argtwo;argfour = functioncall(1,2);argfour  "
            )
        )
    );
}
