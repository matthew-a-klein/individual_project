use crate::{
    evaluator::evaluator::eval_exp, lexer::tokeniser::tokenise, parsers::main_parser::parse_all,
};
mod evaluator;
mod expressions;
mod lexer;
mod parsers;
mod regex;
mod tokens;
fn main() -> () {
    println!("{:?}", eval_exp(parse_all(tokenise("(8)"))))
}
