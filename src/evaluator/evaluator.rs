use std::{ collections::HashMap, io::ErrorKind, ops };

use chrono::{ prelude::*, Duration };

use crate::expressions::expressions::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Date(DateTime<Utc>),
    Time(Duration),
    Number(i32),
    Boolean(bool),
}

pub fn evaluate(prog: Vec<Expression>) -> Result<ReturnType, ErrorKind> {
    let result = eval_prog(prog, &HashMap::new(), &HashMap::new())?;
    Ok(result)
}

fn eval_prog(
    prog: Vec<Expression>,
    env: &HashMap<String, ReturnType>,
    funs: &HashMap<String, (Vec<Expression>, Box<Expression>)>
) -> Result<ReturnType, ErrorKind> {
    match &prog[0] {
        Expression::AssignExp { .. } => {
            let (new_vars, new_funs) = eval_stmt(&prog[0], env, funs);
            eval_prog(prog[1..].to_vec(), &new_vars, &new_funs)
        }
        _ => eval_exp(&prog[0], env, funs),
    }
}
/*
Evaluate code snippets that do not return a value.
 */
pub fn eval_stmt(
    exp: &Expression,
    vars: &HashMap<String, ReturnType>,
    funs: &HashMap<String, (Vec<Expression>, Box<Expression>)>
) -> (HashMap<String, ReturnType>, HashMap<String, (Vec<Expression>, Box<Expression>)>) {
    match &exp {
        Expression::AssignExp { left, right } => {
            match *left.clone() {
                Expression::VarExp(name) => {
                    let value = eval_exp(right, vars, funs);
                    if value.is_ok() {
                        let mut new_vars = vars.clone();
                        new_vars.insert(name, value.unwrap());
                        (new_vars, funs.clone())
                    } else {
                        panic!()
                    }
                }
                Expression::CallExp { name, args } => {
                    let mut new_funs = funs.clone();
                    new_funs.insert(name, (args, right.clone()));
                    (vars.clone(), new_funs)
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

/*
Evaluate code snippets that do return a value
*/
fn eval_exp(
    exp: &Expression,
    vars: &HashMap<String, ReturnType>,
    funs: &HashMap<String, (Vec<Expression>, Box<Expression>)>
) -> Result<ReturnType, ErrorKind> {
    match exp {
        Expression::TimeExp(t) => Ok(ReturnType::Time(*t)),
        Expression::DateExp(d) => Ok(ReturnType::Date(*d)),
        Expression::NumberExp(n) => Ok(ReturnType::Number(*n)),
        Expression::InfixExp { left, op, right } => {
            let left = eval_exp(left, vars, funs)?;
            let right = eval_exp(right, vars, funs)?;
            match op.as_str() {
                "+" => { left + right }
                "*" => { left * right }
                "-" => { left - right }
                "/" => { left / right }

                "==" => { Ok(ReturnType::Boolean(left == right)) }
                "<" => { Ok(ReturnType::Boolean(left < right)) }
                _ => Err(ErrorKind::InvalidInput),
            }
        }
        Expression::VarExp(s) => Ok(vars.get(s).unwrap().clone()),

        Expression::ConditionalExp { condition, if_branch, else_branch } => {
            let condition_truth = eval_exp(condition, vars, funs);
            if let Ok(ReturnType::Boolean(truth)) = condition_truth {
                if truth {
                    eval_exp(if_branch, vars, funs)
                } else {
                    eval_exp(else_branch, vars, funs)
                }
            } else {
                Err(ErrorKind::InvalidInput)
            }
        }
        Expression::CallExp { name, args } => {
            if let Some((arguments, body)) = funs.get(name) {
                if arguments.len() != args.len() {
                    return Err(ErrorKind::InvalidInput);
                }

                let mut new_vars = vars.clone();

                for (arg_name, arg_exp) in arguments.iter().zip(args.iter()) {
                    if let Expression::VarExp(name) = arg_name {
                        let value = eval_exp(arg_exp, vars, funs)?;
                        new_vars.insert(name.clone(), value);
                    } else {
                        return Err(ErrorKind::InvalidInput);
                    }
                }

                eval_exp(&body, &new_vars, funs)
            } else {
                Err(ErrorKind::InvalidInput)
            }
        }
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

impl PartialOrd for ReturnType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self, &other) {
            (ReturnType::Date(d1), ReturnType::Date(d2)) => Some(d1.cmp(d2)),

            (ReturnType::Time(t1), ReturnType::Time(t2)) => Some(t1.cmp(t2)),

            (ReturnType::Number(n1), ReturnType::Number(n2)) => Some(n1.cmp(n2)),

            _ => None,
        }
    }
}
