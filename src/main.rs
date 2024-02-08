use crate::{
    interpreter::interpreter::eval_exp, lexer::tokeniser::tokenise, parser::parser::parse_all,
};

mod interpreter;
mod lexer;
mod new_parser;
mod parser;
mod regex;
mod tokens;
mod typer;
fn main() -> () {
    println!(
        "\n \n {:?}",
        eval_exp(parse_all(tokenise("29//02//2000 - 3m "))[0].clone())
    )
}
