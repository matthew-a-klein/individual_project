mod precedence;

use precedence::{final_precedence, get_precedence};
mod tokens;
use tokens::{Operator, Token};

// Import specific variants of the Operator enum for brevity
use Operator::*;

pub fn pratt(limit: i32, tokens: Vec<Token>) -> (Vec<Token>, Vec<Token>) {
    if tokens.len() == 0 {
        (tokens, vec![])
    } else {
        match &tokens[0] {
            Token::LParen => {
                let (left, rest) = pratt(0, tokens[1..].to_vec());
                if rest[0] != Token::RParen {
                    panic!("Error")
                } else {
                    ploop(left, rest[1..].to_vec(), limit)
                }
            }

            _ => ploop(vec![tokens[0].clone()], tokens[1..].to_vec(), limit),
        }
    }
}

pub fn ploop(left: Vec<Token>, right: Vec<Token>, prec_limit: i32) -> (Vec<Token>, Vec<Token>) {
    if right.is_empty() {
        (left, right)
    } else {
        match &right[0] {
            Token::Operator(op) => {
                if get_precedence(op) <= prec_limit {
                    (left, right)
                } else {
                    let (new_right, tokensafter) = pratt(get_precedence(op), right[1..].to_vec());
                    ploop(
                        vec![Token::Binop {
                            left: left,
                            operator: op.clone(),
                            right: new_right,
                        }],
                        tokensafter,
                        final_precedence(op),
                    )
                }
            }
            _ => (left, right),
        }
    }
}

fn main() -> () {
    print!(
        "{:?}",
        pratt(
            0,
            vec![
                Token::Number(1),
                Token::Operator(ADD),
                Token::Number(2),
                Token::Operator(MULTIPLY),
                Token::LParen,
                Token::Number(3),
                Token::Operator(ADD),
                Token::Number(7),
                Token::RParen,
                Token::Operator(POWER),
                Token::Number(3),
            ],
        )
    );
}
