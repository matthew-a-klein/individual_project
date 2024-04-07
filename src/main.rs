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
    println!("{:?}", tokenise("m = 12"));
}
