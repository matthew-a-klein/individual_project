use crate::{tokens::token_classification::*, tokens::tokens::Token};

fn pratt(limit: i32, tokens: Vec<Token>) -> (Vec<Token>, Vec<Token>) {
    println!("Entered Pratt");

    println!("Tokens: {:?}", tokens);
    println!("Limit: {:?}", limit);
    if tokens.len() == 0 {
        (tokens, Vec::new())
    } else {
        ploop(vec![tokens[0].clone()], tokens[1..].to_vec(), limit)
    }
}

fn ploop(left: Vec<Token>, right: Vec<Token>, prec_limit: i32) -> (Vec<Token>, Vec<Token>) {
    println!("Entered Ploop");
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);
    println!("Limit: {:?}", prec_limit);
    if right.is_empty() {
        (left, right)
    } else {
        match &right[0] {
            tok if is_operator(tok) && get_precedence(tok) > prec_limit => {
                //if the operator is infix parse the operator, taking tokens to the left and right as arguments.
                if is_infix(&right[0]) {
                    let (new_right, tokensafter) =
                        pratt(get_precedence(&right[0]), right[1..].to_vec());

                    ploop(
                        vec![Token::InfixOp {
                            left: left,
                            op: Box::new(tok.clone()),
                            right: new_right,
                        }],
                        tokensafter,
                        prec_limit,
                    )
                } else if is_postfix(&right[0]) {
                    ploop(
                        vec![Token::PostfixOp {
                            operand: left,
                            op: Box::new(tok.clone()),
                        }],
                        right[1..].to_vec(),
                        prec_limit,
                    )
                } else if is_prefix(&right[0]) {
                    let (new_right, tokensafter) =
                        pratt(get_precedence(&right[0]), right[1..].to_vec());

                    ploop(
                        vec![Token::PrefixOp {
                            op: Box::new(tok.clone()),
                            right: new_right,
                        }],
                        tokensafter,
                        prec_limit,
                    )
                } else {
                    unreachable!()
                }
            }

            _ => (left, right),
        }
    }
}

pub fn parse(toks: Vec<Token>) -> (Vec<Token>, Vec<Token>) {
    pratt(0, toks)
}

pub fn parse_all(toks: Vec<Token>) -> Vec<Token> {
    let (parsed, unparsed) = parse(toks);
    if unparsed.is_empty() {
        parsed
    } else {
        panic!("Your programme could not be parsed")
    }
}
