use std::io::ErrorKind;

use chrono::Duration;

use crate::{ expressions::expressions::Expression, tokens::tokens::{ Time, Token::{ self, * } } };

use super::main_parser::parse_infix;

pub fn parse_time(
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    if let TimeLength(t) = &tokens[0] {
        let date: Expression = Expression::TimeExp(time_from_string(t));
        parse_infix(date, tokens[1..].to_vec(), prec_limit)
    } else {
        panic!()
    }
}

fn time_from_string(t: &Time) -> Duration {
    match t {
        Time::Second => Duration::seconds(1),
        Time::Minute => Duration::minutes(1),
        Time::Hour => Duration::hours(1),
        Time::Day => Duration::days(1),
        Time::Week => Duration::weeks(1),
        Time::Month => Duration::days(30),
        Time::Year => Duration::days(365),
    }
}
