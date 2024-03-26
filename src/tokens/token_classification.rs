use crate::{ tokens::tokens::Token, tokens::tokens::Token::* };

pub fn get_precedence(t: &Token) -> i32 {
    match t {
        Semi => 0,
        Operator(s) if s.as_str() == "=" || s.as_str() == ":" => 1,
        Operator(s) if s.as_str() == "==" || s.as_str() == ">" || s.as_str() == "<" => 2,
        Operator(s) if s.as_str() == "?" => 2,
        Operator(s) if s.as_str() == "+" || s.as_str() == "-" => 3,
        Operator(s) if s.as_str() == "*" || s.as_str() == "/" => 4,

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
