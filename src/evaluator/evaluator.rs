use std::{ collections::HashMap, fmt, io::ErrorKind, ops };

use chrono::{ prelude::*, Duration };

use crate::expressions::expressions::Expression;

// Wrapper function, just takes the list of expressions
pub fn evaluate(prog: Vec<Expression>) -> Result<ReturnType, ErrorKind> {
    let result = eval_prog(prog, &HashMap::new(), &HashMap::new())?;
    Ok(result)
}

// Evaluates programmes by iterating through a list of expressions,
// Evaluate assignments with eval_stmt
// Evaluate values with eval_exp, then return that value
fn eval_prog(
    prog: Vec<Expression>,
    env: &HashMap<String, ReturnType>,
    funs: &HashMap<String, (Vec<Expression>, Box<Expression>)>
) -> Result<ReturnType, ErrorKind> {
    match &prog[0] {
        Expression::AssignExp { .. } => {
            let (new_vars, new_funs) = eval_stmt(&prog[0], env, funs)?;
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
) -> Result<
    (HashMap<String, ReturnType>, HashMap<String, (Vec<Expression>, Box<Expression>)>),
    ErrorKind
> {
    match &exp {
        Expression::AssignExp { left, right } => {
            match *left.clone() {
                Expression::VarExp(name) => {
                    let value = eval_exp(right, vars, funs)?;

                    let mut new_vars = vars.clone();
                    new_vars.insert(name, value);
                    Ok((new_vars, funs.clone()))
                }
                Expression::CallExp { name, args } => {
                    let mut new_funs = funs.clone();
                    new_funs.insert(name, (args, right.clone()));
                    Ok((vars.clone(), new_funs))
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
                // Mathematical operations
                "+" => { left + right }
                "*" => { left * right }
                "-" => { left - right }
                "/" => { left / right }
                // Boolean comparison operators
                "==" => { Ok(ReturnType::Boolean(left == right)) }
                "<" => { Ok(ReturnType::Boolean(left < right)) }
                ">" => { Ok(ReturnType::Boolean(left > right)) }
                "<=" => { Ok(ReturnType::Boolean(left <= right)) }
                ">=" => { Ok(ReturnType::Boolean(left >= right)) }
                _ => Err(ErrorKind::InvalidInput),
            }
        }
        Expression::VarExp(s) => {
            let value = vars.get(s);
            match value {
                None => Err(ErrorKind::InvalidInput),
                Some(val) => Ok(val.clone()),
            }
        }

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

//  Represents the kind of values that can be returned
#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Date(DateTime<Utc>),
    Time(Duration),
    Number(i32),
    Boolean(bool),
}

//  Pretty prints returned values to cli
impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnType::Date(date_time) => write!(f, "{}", date_time.format("%d/%m/%Y %H:%M:%S")),
            ReturnType::Time(duration) => {
                let days = duration.num_days();
                let (years, remaining_days) = (days / 365, days % 365);
                let (hours, _) = (duration.num_hours() % 24, duration.num_minutes() % 60);
                let (minutes, seconds) = (duration.num_minutes() % 60, duration.num_seconds() % 60);
                if years > 0 {
                    write!(
                        f,
                        "{} years, {} days, {:02}:{:02}:{:02}",
                        years,
                        remaining_days,
                        hours,
                        minutes,
                        seconds
                    )
                } else {
                    write!(f, "{} days, {:02}:{:02}:{:02}", remaining_days, hours, minutes, seconds)
                }
            }
            ReturnType::Number(num) => write!(f, "{}", num),
            ReturnType::Boolean(boolean) => write!(f, "{}", boolean),
        }
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

// Means that we can do less than or more then comparisons on certain types
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
