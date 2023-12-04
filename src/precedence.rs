use crate::tokens::Operator;

pub fn get_precedence(t: &Operator) -> i32 {
    match t {
        Operator::ADD => 1,
        Operator::SUB => 1,
        Operator::MULTIPLY => 2,
        Operator::DIVIDE => 2,
        Operator::POWER => 3,
    }
}

pub fn is_right_assoc(t: &Operator) -> bool {
    match t {
        Operator::POWER => true,
        _ => false,
    }
}

pub fn final_precedence(t: &Operator) -> i32 {
    if is_right_assoc(t) {
        get_precedence(t) - 1
    } else {
        get_precedence(t)
    }
}
