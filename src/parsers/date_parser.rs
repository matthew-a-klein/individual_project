// This function parses a date expression from the given tokens.

use std::io::ErrorKind;

use chrono::{ DateTime, Utc };

use crate::{ expressions::expressions::Expression, tokens::tokens::Token::{ self, * } };

use super::main_parser::parse_infix;

/// Parses a date expression.
pub fn parse_date(
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    // Ensure the first token is a Date
    if let Date(s) = &tokens[0] {
        // Parse the date string and create a DateTime object
        let date: Expression = Expression::DateExp(date_from_string(s.to_string()));
        // Continue parsing infix expressions with the parsed date expression
        parse_infix(date, tokens[1..].to_vec(), prec_limit)
    } else {
        // Panic if the first token is not a Date
        panic!("Unexpected token, expected Date")
    }
}

/// Converts a date string into a DateTime object.
fn date_from_string(s: String) -> DateTime<Utc> {
    DateTime::parse_from_str(
        format!("{s} 00:00:00.000 +0000").as_str(),
        "%d//%m//%Y %H:%M:%S%.3f %z"
    )
        .unwrap()
        .to_utc()
}
