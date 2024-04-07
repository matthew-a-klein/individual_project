use crate::{ expressions::expressions::Expression::{ self }, tokens::tokens::Token::{ self, * } };

use super::main_parser::{ parse_infix, parse_prefix };

pub fn parse_group(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> (Expression, Vec<Token>) {
    if let Expression::VarExp(s) = left {
        if let LParen(_) = &tokens[0] {
            let (args, tokens_after) = parse_args(Vec::new(), tokens[1..].to_vec());
            if let RParen(_) = &tokens_after[0] {
                let call_exp = Expression::CallExp { name: s, args };
                parse_infix(call_exp, tokens_after[1..].to_vec(), prec_limit)
            } else {
                panic!("")
            }
        } else {
            panic!("")
        }
    } else {
        panic!()
    }
}

fn parse_args(mut args: Vec<Expression>, toks: Vec<Token>) -> (Vec<Expression>, Vec<Token>) {
    match &toks[0] {
        RParen(_) => (args, toks),
        Operator(s) if s.as_str() == "," => parse_args(args, toks[1..].to_vec()),
        _ => {
            let (new_arg, tokens_after) = parse_prefix(0, toks);
            args.push(new_arg);
            parse_args(args, tokens_after)
        }
    }
}
