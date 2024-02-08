use std::{fmt::Error, io::ErrorKind};

use chrono::{prelude::*, Duration};

use crate::{
    tokens::tokens::{Time, Token, Type::*},
    typer::typer::type_tok,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TimeType {
    Date(DateTime<Utc>),
    Time(Duration),
}

pub fn eval_exp(tok: Token) -> Result<TimeType, ErrorKind> {
    match &tok {
        Token::Date(_) => eval_date(tok),
        Token::TimeLength(_) => eval_time(tok),
        Token::InfixOp { left, op, right } => match (type_tok(&left[0]), type_tok(&right[0])) {
            (Time, Time) => eval_time(tok),
            (Time, Date) => unreachable!(),
            (Date, Time) => eval_date(tok),
            (Date, Date) => eval_time(tok),
        },
        Token::PostfixOp { operand, op } => eval_time(tok),
        Token::PrefixOp { op, right } => unreachable!(),
        _ => unreachable!(),
    }
}

pub fn eval_date(tok: Token) -> Result<TimeType, ErrorKind> {
    match tok {
        Token::Date(s) => {
            let date = DateTime::parse_from_str(
                format!("{s} 00:00:00.000 +0000").as_str(),
                "%d//%m//%Y %H:%M:%S%.3f %z",
            );

            if date.is_ok() {
                Ok(TimeType::Date(date.unwrap().to_utc()))
            } else {
                Err(ErrorKind::InvalidInput)
            }
        }

        Token::InfixOp { left, op, right } => match (type_tok(&left[0]), type_tok(&right[0])) {
            (Date, Time) => {
                let date = eval_date(left[0].clone());
                let duration = eval_time(right[0].clone());

                if let (TimeType::Date(date), TimeType::Time(duration)) =
                    (date.unwrap(), duration.unwrap())
                {
                    match &*op {
                        Token::Operator(s) => match s.as_str() {
                            "+" => Ok(TimeType::Date(date + duration)),
                            "-" => Ok(TimeType::Date(date - duration)),
                            _ => Err(ErrorKind::InvalidInput),
                        },
                        _ => Err(ErrorKind::InvalidInput),
                    }
                } else {
                    Err(ErrorKind::InvalidInput)
                }
            }
            _ => Err(ErrorKind::InvalidInput),
        },
        _ => Err(ErrorKind::InvalidInput),
    }
}

pub fn eval_time(tok: Token) -> Result<TimeType, ErrorKind> {
    match &tok {
        Token::TimeLength(t) => todo!(),
        Token::InfixOp { left, op, right } => match (type_tok(&left[0]), type_tok(&right[0])) {
            (Time, Time) => todo!(),

            (Date, Date) => {
                let date_1 = eval_date(left[0].clone());
                let date_2 = eval_date(right[0].clone());
                if let (TimeType::Date(date_1), TimeType::Date(date_2)) =
                    (date_1.unwrap(), date_2.unwrap())
                {
                    match &**op {
                        Token::Operator(s) => match s.as_str() {
                            "-" => Ok(TimeType::Time(date_1 - date_2)),
                            _ => Err(ErrorKind::InvalidInput),
                        },
                        _ => Err(ErrorKind::InvalidInput),
                    }
                } else {
                    Err(ErrorKind::InvalidInput)
                }
            }
            _ => unreachable!(),
        },
        Token::PostfixOp { operand, op } => match &**op {
            Token::TimeLength(h) => {
                if let Token::Number(n) = operand[0] {
                    match h {
                        Time::Second => Ok(TimeType::Time(Duration::seconds(n.into()))),
                        Time::Minute => Ok(TimeType::Time(Duration::minutes(n.into()))),
                        Time::Hour => Ok(TimeType::Time(Duration::hours(n.into()))),
                        Time::Day => Ok(TimeType::Time(Duration::days(n.into()))),
                        Time::Week => Ok(TimeType::Time(Duration::weeks(n.into()))),

                        _ => Err(ErrorKind::Unsupported),
                    }
                } else {
                    Err(ErrorKind::InvalidInput)
                }
            }
            _ => Err(ErrorKind::InvalidInput),
        },
        Token::PrefixOp { .. } => unreachable!(),
        _ => unreachable!(),
    }
}
