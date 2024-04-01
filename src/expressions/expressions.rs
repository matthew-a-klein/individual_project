use chrono::{ prelude::*, Duration };
use std::fmt::{ self, write };
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
        left: Box<Expression>,
        right: Box<Expression>,
    },
    ConditionalExp {
        condition: Box<Expression>,
        if_branch: Box<Expression>,
        else_branch: Box<Expression>,
    },
    CallExp {
        name: String,
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
            Expression::VarExp(s) => write!(f, "{s}"),
            Expression::InfixExp { left, op, right } => {
                write!(f, "({:?} {} {:?})", left, op, right)
            }
            Expression::PostfixExp { left, op } => write!(f, "({:?} {:?})", left, op),
            Expression::PrefixExp { op, right } => write!(f, "({:?} {:?})", op, right),
            Expression::AssignExp { left, right } => write!(f, "({:?} = {:?})", left, right),
            Expression::ConditionalExp { condition, if_branch, else_branch } =>
                write!(f, "if {:?} then ({:?}) else ({:?})", condition, if_branch, else_branch),
            Expression::CallExp { name, args } => { write!(f, "{}({:?})", name, args) }
            _ => write!(f, "Not implemented"),
        }
    }
}
