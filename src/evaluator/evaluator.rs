use std::{collections::HashMap, io::ErrorKind, ops};

use chrono::{prelude::*, Duration};

use crate::expressions::expressions::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Date(DateTime<Utc>),
    Time(Duration),
    Number(i32),
}

pub fn eval_prog(
    prog: Vec<Expression>,
    env: &HashMap<String, ReturnType> 
) -> Result<ReturnType, ErrorKind>{
    match &prog[0]{
     Expression::AssignExp { name, right } => 
     {
        let new_env = eval_stmt(&prog[0], env);
        eval_prog(prog[1..].to_vec(), &new_env) 
     }
     ,
     _ => eval_exp(&prog[0], env)  
    }
}
/*
Evaluate code snippets that do not return a value.
 */
pub fn eval_stmt(
    exp: &Expression,
    env: &HashMap<String, ReturnType>,
) -> HashMap<String, ReturnType> {
    match &exp {
        Expression::AssignExp { name, right } => {
            let value = eval_exp(right, env);
            if value.is_ok() {
                let mut new_env = env.clone();
                new_env.insert(name.clone(), value.unwrap());
                new_env
            } else {
                panic!()
            }
        }
        _ => unreachable!(),
    }
}
/*
Evaluate code snippets that do return a value
*/
pub fn eval_exp(
    exp: &Expression,
    env: &HashMap<String, ReturnType>,
) -> Result<ReturnType, ErrorKind> {
    match exp {
        Expression::TimeExp(t) => Ok(ReturnType::Time(*t)),
        Expression::DateExp(d) => Ok(ReturnType::Date(*d)),
        Expression::NumberExp(n) => Ok(ReturnType::Number(*n)),
        Expression::InfixExp { left, op, right } => match (op).as_str() {
            "+" => {
                let eval_left = eval_exp(left, env);
                let eval_right = eval_exp(right, env);

                if let (Ok(l), Ok(r)) = (eval_left, eval_right) {
                    l + r
                } else {
                    Err(ErrorKind::InvalidInput)
                }
            }
            "*" => {
                let eval_left = eval_exp(left, env);
                let eval_right = eval_exp(right, env);

                if let (Ok(l), Ok(r)) = (eval_left, eval_right) {
                    l * r
                } else {
                    Err(ErrorKind::InvalidInput)
                }
            }
            "-" => {
                let eval_left = eval_exp(left, env);
                let eval_right = eval_exp(right, env);

                if let (Ok(l), Ok(r)) = (eval_left, eval_right) {
                    l - r
                } else {
                    Err(ErrorKind::InvalidInput)
                }
            }
            "/" => {
                let eval_left = eval_exp(left, env);
                let eval_right = eval_exp(right, env);

                if let (Ok(l), Ok(r)) = (eval_left, eval_right) {
                    l / r
                } else {
                    Err(ErrorKind::InvalidInput)
                }
            }
            _ => Err(ErrorKind::InvalidInput),
        },
        Expression::VarExp(s) => Ok(env.get(s).unwrap().clone()),
        _ => unimplemented!(),
    }
}

// Define the mathematical operations for times and dates.

// The "+" operator
impl ops::Add<ReturnType> for ReturnType {
    type Output = Result<ReturnType, ErrorKind>;

    fn add(self, _rhs: ReturnType) -> Result<ReturnType, ErrorKind> {
        match (&self, &_rhs) {
            (ReturnType::Date(d), ReturnType::Time(t)) => Ok(ReturnType::Date(*d + *t)),
            (ReturnType::Time(t), ReturnType::Date(d)) => Ok(ReturnType::Date(*d + *t)),
            (ReturnType::Time(t1), ReturnType::Time(t2)) => Ok(ReturnType::Time(*t1 + *t2)),
            (ReturnType::Number(n1), ReturnType::Number(n2)) => Ok(ReturnType::Number(*n1 + *n2)),
            _ => Err(ErrorKind::InvalidInput),
        }
    }
}
// The "*" operator
impl ops::Mul<ReturnType> for ReturnType {
    type Output = Result<ReturnType, ErrorKind>;

    fn mul(self, _rhs: ReturnType) -> Result<ReturnType, ErrorKind> {
        match (&self, &_rhs) {
            (ReturnType::Time(t), ReturnType::Number(n)) => Ok(ReturnType::Time(*t * *n)),

            (ReturnType::Number(n), ReturnType::Time(t)) => Ok(ReturnType::Time(*t * *n)),
            (ReturnType::Number(n1), ReturnType::Number(n2)) => Ok(ReturnType::Number(*n1 * *n2)),
            _ => Err(ErrorKind::InvalidInput),
        }
    }
}
// The "-" operator
impl ops::Sub for ReturnType {
    type Output = Result<ReturnType, ErrorKind>;

    fn sub(self, _rhs: ReturnType) -> Result<ReturnType, ErrorKind> {
        match (&self, &_rhs) {
            (ReturnType::Date(d1), ReturnType::Date(d2)) => Ok(ReturnType::Time(*d1 - *d2)),
            (ReturnType::Date(d), ReturnType::Time(t)) => Ok(ReturnType::Date(*d - *t)),

            (ReturnType::Time(t1), ReturnType::Time(t2)) => Ok(ReturnType::Time(*t1 - *t2)),

            (ReturnType::Number(n1), ReturnType::Number(n2)) => Ok(ReturnType::Number(*n1 - *n2)),
            _ => Err(ErrorKind::InvalidInput),
        }
    }
}

// The "/" operator
impl ops::Div for ReturnType {
    type Output = Result<ReturnType, ErrorKind>;

    fn div(self, _rhs: ReturnType) -> Result<ReturnType, ErrorKind> {
        match (&self, &_rhs) {
            (ReturnType::Time(t), ReturnType::Number(n)) => Ok(ReturnType::Time(*t / *n)),

            (ReturnType::Number(n1), ReturnType::Number(n2)) => Ok(ReturnType::Number(*n1 / *n2)),
            _ => Err(ErrorKind::InvalidInput),
        }
    }
}
