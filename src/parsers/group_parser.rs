use std::io::ErrorKind;

use crate::{ expressions::expressions::Expression::{ self }, tokens::tokens::Token::{ self, * } };

use super::main_parser::{ parse_infix, parse_prefix };

// Parses a grouped expression, typically a function call.
pub fn parse_group(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    if let Expression::VarExp(s) = left {
        if let LParen(_) = &tokens[0] {
            let (args, tokens_after) = parse_args(Vec::new(), tokens[1..].to_vec())?;
            if let RParen(_) = &tokens_after[0] {
                let call_exp = Expression::CallExp { name: s, args };
                parse_infix(call_exp, tokens_after[1..].to_vec(), prec_limit)
            } else {
                Err(ErrorKind::InvalidInput)
            }
        } else {
            panic!("")
        }
    } else {
        Err(ErrorKind::InvalidInput)
    }
}

// Helper function to parse arguments inside parentheses.
fn parse_args(
    mut args: Vec<Expression>,
    toks: Vec<Token>
) -> Result<(Vec<Expression>, Vec<Token>), ErrorKind> {
    match &toks[0] {
        RParen(_) => Ok((args, toks)),
        Operator(s) if s.as_str() == "," => parse_args(args, toks[1..].to_vec()),
        _ => {
            let (new_arg, tokens_after) = parse_prefix(0, toks)?;
            args.push(new_arg);
            parse_args(args, tokens_after)
        }
    }
}
