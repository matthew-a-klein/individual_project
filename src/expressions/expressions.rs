use chrono::{prelude::*, Duration};
use std::fmt;
#[derive(Clone, PartialEq)]
pub enum Expression {
    TimeExp(Duration),
    DateExp(DateTime<Utc>),
    NumberExp(i32),
    VarExp(String),
    InfixExp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
    PostfixExp {
        left: Box<Expression>,
        op: String,
    },
    PrefixExp {
        op: String,
        right: Box<Expression>,
    },
    AssignExp {
        name: String,
        right: Box<Expression>,
    },
    NameExp {
        name: String,
    },
    CondititionalExp {
        condition: Box<Expression>,
        if_branch: Box<Expression>,
        else_branch: Box<Expression>,
    },
    CallExp {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    Empty,
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Expression::TimeExp(t) => write!(f, "{:?}", t),
            Expression::DateExp(d) => write!(f, "{:?}", d),
            Expression::NumberExp(n) => write!(f, "{:?}", n),
            Expression::VarExp(s) => write!(f, "{:?}", s),
            Expression::InfixExp { left, op, right } => {
                write!(f, "({:?}, {:?}, {:?})", left, op, right)
            }
            Expression::PostfixExp { left, op } => write!(f, "({:?} {:?})", left, op),
            Expression::PrefixExp { op, right } => write!(f, "( {:?} {:?})", op, right),
            Expression::AssignExp { name, right } => write!(f, "( {:?} = {:?})", name, right),
            _ => write!(f, "Not implemented"),
        }
    }
}
