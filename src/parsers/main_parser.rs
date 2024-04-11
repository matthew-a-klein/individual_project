// This module provides parsers for different types of expressions and operators,
// enabling the parsing of a program's tokens into an abstract syntax tree (AST).

use std::io::ErrorKind;

use crate::{
    expressions::expressions::Expression,
    parsers::{
        assignment_parser::parse_assignment,
        conditional_parser::parse_conditional,
        date_parser::parse_date,
        call_parser::parse_call,
        number_parser::parse_number,
        postfix_parser::parse_postfix,
        prefix_parser::parse_prefix_op,
        time_parser::parse_time,
    },
    tokens::{ token_classification::get_precedence, tokens::Token },
};

use super::{ binop_parser::parse_binop, paren_parser::parse_paren, var_parser::parse_var };

/// Parses a prefix expression from the given tokens up to a specified limit of precedence.
pub fn parse_prefix(limit: i32, tokens: Vec<Token>) -> Result<(Expression, Vec<Token>), ErrorKind> {
    if tokens.len() == 0 {
        Ok((Expression::Empty, Vec::new()))
    } else {
        match &tokens[0] {
            Token::Number(_) => Ok(parse_number(tokens, limit)?),
            Token::Operator(_) => Ok(parse_prefix_op(tokens, limit)?),
            Token::TimeLength(_) => Ok(parse_time(tokens, limit)?),
            Token::Date(_) => Ok(parse_date(tokens, limit)?),
            Token::LParen(_) => Ok(parse_paren(tokens, limit)?),
            Token::Var(_) => Ok(parse_var(tokens, limit)?),
            _ => Err(ErrorKind::InvalidInput),
        }
    }
}

/// Parses an infix expression by combining it with the left expression, up to a specified precedence limit.
pub fn parse_infix(
    left: Expression,
    tokens: Vec<Token>,
    prec_limit: i32
) -> Result<(Expression, Vec<Token>), ErrorKind> {
    if tokens.len() == 0 || get_precedence(&tokens[0]) <= prec_limit {
        Ok((left, tokens))
    } else {
        match &tokens[0] {
            Token::Semi => Ok((left, tokens[1..].to_vec())),
            Token::Operator(s) if s == "=" => Ok(parse_assignment(left, tokens, prec_limit)?),
            Token::Operator(s) if s == "?" => Ok(parse_conditional(left, tokens, prec_limit)?),
            Token::Operator(s) if s == ":" => Ok((left, tokens)),
            Token::Operator(_) => Ok(parse_binop(left, tokens, prec_limit)?),
            Token::LParen(_) => Ok(parse_call(left, tokens, prec_limit)?),
            Token::RParen(_) => Ok((left, tokens)),
            _ => Err(ErrorKind::InvalidInput),
        }
    }
}

/// Parses a sequence of expressions from the given tokens.
pub fn parse_expressions(
    mut expressions: Vec<Expression>,
    toks: Vec<Token>
) -> Result<(Vec<Expression>, Vec<Token>), ErrorKind> {
    if toks.is_empty() {
        Ok((expressions, toks))
    } else {
        let (expression, remaining_tokens) = parse_prefix(0, toks)?;
        expressions.extend(vec![expression]);
        if !remaining_tokens.is_empty() && remaining_tokens[0] == Token::Semi {
            parse_expressions(expressions, remaining_tokens[1..].to_vec())
        } else {
            parse_expressions(expressions, remaining_tokens)
        }
    }
}

/// Parses a single expression from the given tokens.
pub fn parse(toks: Vec<Token>) -> Result<(Expression, Vec<Token>), ErrorKind> {
    parse_prefix(0, toks)
}

/// Parses all tokens into a single expression.
pub fn parse_all(toks: Vec<Token>) -> Result<Expression, ErrorKind> {
    let (parsed, unparsed) = parse(toks)?;
    if unparsed.is_empty() {
        Ok(parsed)
    } else {
        Err(ErrorKind::InvalidInput)
    }
}

/// Parses a sequence of expressions representing a program.
pub fn parse_programme(toks: Vec<Token>) -> Result<Vec<Expression>, ErrorKind> {
    let (parsed, unparsed) = parse_expressions(Vec::new(), toks)?;
    if unparsed.is_empty() {
        Ok(parsed)
    } else {
        Err(ErrorKind::InvalidInput)
    }
}
