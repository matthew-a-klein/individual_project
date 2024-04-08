use std::io::ErrorKind;

use chrono::{ DateTime, Utc };

use crate::{ expressions::expressions::Expression, tokens::tokens::Token::{ self, * } };

use super::main_parser::parse_infix;

pub fn parse_date(
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    if let Date(s) = &tokens[0] {
        let date: Expression = Expression::DateExp(date_from_string(s.to_string()));
        parse_infix(date, tokens[1..].to_vec(), prec_limit)
    } else {
        panic!()
    }
}

fn date_from_string(s: String) -> DateTime<Utc> {
    DateTime::parse_from_str(
        format!("{s} 00:00:00.000 +0000").as_str(),
        "%d//%m//%Y %H:%M:%S%.3f %z"
    )
        .unwrap()
        .to_utc()
}
