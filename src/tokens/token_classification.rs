use crate::{ tokens::tokens::Token, tokens::tokens::Token::* };

// Function to determine the precedence of a token in the context of expressions.
pub fn get_precedence(t: &Token) -> i32 {
    match t {
        // Lowest precedence for semicolons and commas.
        Semi => 0,
        Operator(s) if s.as_str() == "," => 0,
        // Assignment and conditional operator have precedence 1.
        Operator(s) if s.as_str() == "=" || s.as_str() == ":" => 1,
        // Comparison and ternary operators have precedence 2.
        Operator(s) if
            s.as_str() == "==" ||
            s.as_str() == ">" ||
            s.as_str() == "<" ||
            s.as_str() == ">=" ||
            s.as_str() == "<=" ||
            s.as_str() == "?"
        => 2,
        // Addition and subtraction have precedence 3.
        Operator(s) if s.as_str() == "+" || s.as_str() == "-" => 3,
        // Multiplication and division have precedence 4.
        Operator(s) if s.as_str() == "*" || s.as_str() == "/" => 4,
        // Parentheses have highest precedence.
        RParen(_) => 0,
        LParen(_) => 10,
        // Unreachable case, as all token variants should be covered above.
        _ => unreachable!(),
    }
}

// Function to determine if a token is right-associative.
pub fn is_right_assoc(t: &Token) -> bool {
    if let Operator(s) = t {
        match s.as_str() {
            // Right-associative operators go here.
            _ => false,
        }
    } else {
        // If the token is not an operator, it shouldn't be passed to this function.
        unreachable!()
    }
}
