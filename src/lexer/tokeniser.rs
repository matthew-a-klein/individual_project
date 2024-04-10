use std::io::ErrorKind;

use crate::{
    lexer::lexer::lexing_simp,
    regex::reg::*,
    tokens::tokens::Time::*,
    tokens::tokens::Token,
    tokens::tokens::Token::*,
};

//  Matches one digit from 0 to 9
fn digit_reg() -> Re {
    char('0') |
        char('1') |
        char('2') |
        char('3') |
        char('4') |
        char('5') |
        char('6') |
        char('7') |
        char('8') |
        char('9')
}

// Matches any upper or lower case letter
fn letter_reg() -> Re {
    char('a') |
        char('A') |
        char('b') |
        char('B') |
        char('c') |
        char('C') |
        char('d') |
        char('D') |
        char('e') |
        char('E') |
        char('f') |
        char('F') |
        char('g') |
        char('G') |
        char('h') |
        char('H') |
        char('i') |
        char('I') |
        char('j') |
        char('J') |
        char('k') |
        char('K') |
        char('l') |
        char('L') |
        char('m') |
        char('M') |
        char('n') |
        char('N') |
        char('o') |
        char('O') |
        char('p') |
        char('P') |
        char('q') |
        char('Q') |
        char('r') |
        char('R') |
        char('s') |
        char('S') |
        char('t') |
        char('T') |
        char('u') |
        char('U') |
        char('v') |
        char('V') |
        char('w') |
        char('W') |
        char('x') |
        char('X') |
        char('y') |
        char('Y') |
        char('z') |
        char('Z')
}

// matches numbers with at least one digit
fn number_reg() -> Re {
    digit_reg() + star(digit_reg())
}

// Matches comments, comments are enclose with /* ... */
fn comment_reg() -> Re {
    string_to_rexp("/*") +
        star(letter_reg() | number_reg() | whitespace_reg() | operator_reg()) +
        string_to_rexp("*/")
}

// Matches variable names
// Variable names have at least one letter, followed by any number of digits
fn variable_reg() -> Re {
    letter_reg() + star(letter_reg()) + star(digit_reg())
}

// Matches whitespace, this will be filtered out by the tokenise function
pub fn whitespace_reg() -> Re {
    star(char('\n') | char(' ') | char('\r') | char('\t'))
}

// Matches hour duration
pub fn hour_reg() -> Re {
    char('h') | char('H') | string_to_rexp("hour") | string_to_rexp("Hour")
}

// Matches minute duration
pub fn minute_reg() -> Re {
    char('m') |
        char('M') |
        string_to_rexp("min") |
        string_to_rexp("Min") |
        string_to_rexp("Minute") |
        string_to_rexp("minute")
}

// Matches second duration
pub fn second_reg() -> Re {
    char('s') |
        char('S') |
        string_to_rexp("sec") |
        string_to_rexp("Sec") |
        string_to_rexp("second") |
        string_to_rexp("Second")
}

// Matches day duration
pub fn day_reg() -> Re {
    char('d') | char('D') | string_to_rexp("day") | string_to_rexp("Day")
}

// Matches week duration
pub fn week_reg() -> Re {
    char('w') | char('W') | string_to_rexp("week") | string_to_rexp("Week")
}

// Matches month duration
pub fn month_time_reg() -> Re {
    string_to_rexp("month") | string_to_rexp("Month")
}

// Matches year duration
pub fn year_time_reg() -> Re {
    char('y') | char('Y') | string_to_rexp("year") | string_to_rexp("Year")
}

// Matches operators
pub fn operator_reg() -> Re {
    char('=') |
        char('+') |
        char('-') |
        char('/') |
        char('*') |
        char('!') |
        char('?') |
        char(':') |
        char('<') |
        char('<') |
        char(',') |
        string_to_rexp("==") |
        string_to_rexp("<=") |
        string_to_rexp(">=")
}

// Matches semicolon, which means the end of the expression
pub fn semi_reg() -> Re {
    char(';')
}

//  Matches dates: dates must be specified with dd//mm//yyyy notation
pub fn date_reg() -> Re {
    number_reg() +
        opt(whitespace_reg()) +
        string_to_rexp("//") +
        opt(whitespace_reg()) +
        number_reg() +
        opt(whitespace_reg()) +
        string_to_rexp("//") +
        opt(whitespace_reg()) +
        number_reg()
}

// Matches left parentheses
pub fn l_paren_reg() -> Re {
    char('(') | char('{')
}

// Matches right parentheses
pub fn r_paren_reg() -> Re {
    char(')') | char('}')
}

//  Matches valid programmes
pub fn prog_reg() -> Re {
    star(
        recd("h", hour_reg()) |
            recd("m", minute_reg()) |
            recd("s", second_reg()) |
            recd("n", number_reg()) |
            recd("op", operator_reg()) |
            recd("w", whitespace_reg()) |
            recd("week", week_reg()) |
            recd("day", day_reg()) |
            recd("month", month_time_reg()) |
            recd("date", date_reg()) |
            recd("year", year_time_reg()) |
            recd("l_paren", l_paren_reg()) |
            recd("r_paren", r_paren_reg()) |
            recd("semi", semi_reg()) |
            recd("var", variable_reg()) |
            recd("comment", comment_reg())
    )
}

//  Maps record regular expressions to corresponding tokens
//  Whitespaces maps to none
pub fn map_to_tokens(s: &(String, String)) -> Option<Token> {
    let (s1, s2) = s;
    match (s1.as_str(), s2) {
        ("h", _) => Some(TimeLength(Hour)),
        ("s", _) => Some(TimeLength(Second)),
        ("m", _) => Some(TimeLength(Minute)),
        ("day", _) => Some(TimeLength(Day)),
        ("week", _) => Some(TimeLength(Week)),
        ("month", _) => Some(TimeLength(Month)),
        ("year", _) => Some(TimeLength(Year)),
        ("date", s) => Some(Date(s.to_string())),
        ("n", s) => Some(Number(s.parse::<i32>().unwrap())),
        ("op", s) => Some(Operator(s.to_string())),
        ("l_paren", s) => Some(LParen(s.to_string())),
        ("r_paren", s) => Some(RParen(s.to_string())),
        ("var", s) => Some(Var(s.to_string())),
        ("semi", _) => Some(Semi),
        _ => None,
    }
}

// Wrapper function
// Takes a programme in string format
// Returns a vector of tokens if valid
// Raises an error if invalid
pub fn tokenise(s: &str) -> Result<Vec<Token>, ErrorKind> {
    Ok(
        lexing_simp(&prog_reg(), &s)?
            .iter()
            .filter_map(|x| map_to_tokens(x))
            .collect()
    )
}
