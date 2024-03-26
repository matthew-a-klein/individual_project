use crate::{
    expressions::expressions::Expression,
    parsers::{
        assignment_parser::parse_assignment,
        conditional_parser::parse_conditional,
        date_parser::parse_date,
        number_parser::parse_number,
        postfix_parser::parse_postfix,
        prefix_parser::parse_prefix_op,
        r_paren_parser::parse_r_paren,
        time_parser::parse_time,
    },
    tokens::{ token_classification::get_precedence, tokens::Token },
};

use super::{ binop_parser::parse_binop, group_parser::parse_group, var_parser::parse_var };

pub fn parse_prefix(limit: i32, tokens: Vec<Token>) -> (Expression, Vec<Token>) {
    println!("Parsing Prefix: {:?} with limit {:?}", tokens, limit);
    if tokens.len() == 0 {
        (Expression::Empty, Vec::new())
    } else {
        match &tokens[0] {
            Token::Number(_) => parse_number(tokens, limit),
            Token::Operator(_) => parse_prefix_op(tokens, limit),
            Token::TimeLength(_) => parse_time(tokens, limit),
            Token::Date(_) => parse_date(tokens, limit),
            Token::LParen(_) => parse_group(tokens, limit),
            Token::Var(_) => parse_var(tokens, limit),
            _ => panic!(),
        }
    }
}

pub fn parse_infix(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> (Expression, Vec<Token>) {
    println!("Parsing Infix: {:?} with limit {:?}", tokens, prec_limit);
    if tokens.len() == 0 || get_precedence(&tokens[0]) <= prec_limit {
        (left, tokens)
    } else {
        match &tokens[0] {
            Token::Semi => (left, tokens[1..].to_vec()),
            Token::Operator(s) if s == "=" => parse_assignment(left, tokens, prec_limit),
            Token::Operator(s) if s == "?" => parse_conditional(left, tokens, prec_limit),
            Token::Operator(s) if s == ":" => (left, tokens),
            Token::Operator(_) => parse_binop(left, tokens, prec_limit),
            Token::RParen(_) => parse_r_paren(left, tokens, prec_limit),
            _ => panic!(),
        }
    }
}

pub fn parse_expressions(
    mut expressions: Vec<Expression>,
    toks: Vec<Token>
) -> (Vec<Expression>, Vec<Token>) {
    if toks.is_empty() {
        (expressions, toks)
    } else {
        let (expression, remaining_tokens) = parse_prefix(0, toks);
        println!("Expression: {:?}", expression);
        expressions.extend(vec![expression]);
        if !remaining_tokens.is_empty() && remaining_tokens[0] == Token::Semi {
            parse_expressions(expressions, remaining_tokens[1..].to_vec())
        } else {
            parse_expressions(expressions, remaining_tokens)
        }
    }
}

pub fn parse(toks: Vec<Token>) -> (Expression, Vec<Token>) {
    parse_prefix(0, toks)
}

pub fn parse_all(toks: Vec<Token>) -> Expression {
    let (parsed, unparsed) = parse(toks);
    if unparsed.is_empty() {
        parsed
    } else {
        panic!("Your programme could not be parsed")
    }
}

pub fn parse_programme(toks: Vec<Token>) -> Vec<Expression> {
    let (parsed, unparsed) = parse_expressions(Vec::new(), toks);
    if unparsed.is_empty() {
        parsed
    } else {
        panic!("Your programme could not be parsed")
    }
}
