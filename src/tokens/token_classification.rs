use crate::{tokens::tokens::Token, tokens::tokens::Token::*};

pub fn is_operator(t: &Token) -> bool {
    match t {
        Operator(_) => true,
        _ => false,
    }
}

pub fn get_precedence(t: &Token) -> i32 {
    match t {
        op if is_operator(op) => match op {
            Operator(s) => match s.as_str() {
                "+" => 1,
                "-" => 1,
                "/" => 3,
                "*" => 2,
                "//" => 4,
                "!" => 5,
                _ => unreachable!(),
            },
            TimeLength(_) => 4,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn is_right_assoc(t: &Token) -> bool {
    if let Operator(s) = t {
        match s.as_str() {
            _ => false,
        }
    } else {
        unreachable!()
    }
}
