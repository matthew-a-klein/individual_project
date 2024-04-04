use crate::{
    evaluator::evaluator::evaluate,
    lexer::tokeniser::tokenise,
    parsers::main_parser::parse_programme,
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
                    "functioncall(argone, argtwo) = argone == argtwo? argone: argtwo;argfour = functioncall(1,2);argfour  "
                ).unwrap()
            )
        )
    );
}
