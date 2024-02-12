use crate::{
    lexer::lexer::lexing_simp, regex::reg::*, tokens::tokens::Time::*, tokens::tokens::Token,
    tokens::tokens::Token::*,
};

fn digit_reg() -> Re {
    char('0')
        | char('1')
        | char('2')
        | char('3')
        | char('4')
        | char('5')
        | char('6')
        | char('7')
        | char('8')
        | char('9')
}

fn number_reg() -> Re {
    star(digit_reg())
}

fn jan_reg() -> Re {
    string_to_rexp("01")
        | string_to_rexp("jan")
        | string_to_rexp("january")
        | string_to_rexp("Jan")
        | string_to_rexp("January")
}

fn feb_reg() -> Re {
    string_to_rexp("02")
        | string_to_rexp("feb")
        | string_to_rexp("february")
        | string_to_rexp("Feb")
        | string_to_rexp("February")
}

fn march_reg() -> Re {
    string_to_rexp("mar")
        | string_to_rexp("march")
        | string_to_rexp("March")
        | string_to_rexp("Mar")
}

fn april_reg() -> Re {
    string_to_rexp("04")
        | string_to_rexp("apr")
        | string_to_rexp("april")
        | string_to_rexp("April")
        | string_to_rexp("Apr")
}

fn may_reg() -> Re {
    string_to_rexp("05") | string_to_rexp("May") | string_to_rexp("may")
}

fn june_reg() -> Re {
    string_to_rexp("06")
        | string_to_rexp("jun")
        | string_to_rexp("June")
        | string_to_rexp("Jun")
        | string_to_rexp("june")
}

fn july_reg() -> Re {
    string_to_rexp("07")
        | string_to_rexp("jul")
        | string_to_rexp("July")
        | string_to_rexp("Jul")
        | string_to_rexp("july")
}

fn aug_reg() -> Re {
    string_to_rexp("08")
        | string_to_rexp("aug")
        | string_to_rexp("August")
        | string_to_rexp("Aug")
        | string_to_rexp("august")
}

fn sept_reg() -> Re {
    string_to_rexp("09")
        | string_to_rexp("sep")
        | string_to_rexp("September")
        | string_to_rexp("Sep")
        | string_to_rexp("september")
}

fn oct_reg() -> Re {
    string_to_rexp("oct")
        | string_to_rexp("October")
        | string_to_rexp("Oct")
        | string_to_rexp("october")
}

fn nov_reg() -> Re {
    string_to_rexp("nov")
        | string_to_rexp("November")
        | string_to_rexp("Nov")
        | string_to_rexp("november")
}

fn dec_reg() -> Re {
    string_to_rexp("dec")
        | string_to_rexp("Dec")
        | string_to_rexp("December")
        | string_to_rexp("december")
}

pub fn month_reg() -> Re {
    jan_reg()
        | feb_reg()
        | march_reg()
        | may_reg()
        | april_reg()
        | june_reg()
        | july_reg()
        | aug_reg()
        | sept_reg()
        | oct_reg()
        | nov_reg()
        | dec_reg()
}

pub fn year_reg() -> Re {
    number_reg()
}

pub fn whitespace_reg() -> Re {
    star(char('\n') | char(' ') | char('\r') | char('\t'))
}

pub fn hour_reg() -> Re {
    char('h') | char('H') | string_to_rexp("hour") | string_to_rexp("Hour")
}

pub fn minute_reg() -> Re {
    char('m')
        | char('M')
        | string_to_rexp("min")
        | string_to_rexp("Min")
        | string_to_rexp("Minute")
        | string_to_rexp("minute")
}

pub fn second_reg() -> Re {
    char('s')
        | char('S')
        | string_to_rexp("sec")
        | string_to_rexp("Sec")
        | string_to_rexp("second")
        | string_to_rexp("Second")
}

pub fn day_reg() -> Re {
    char('d') | char('D') | string_to_rexp("day") | string_to_rexp("Day")
}

pub fn week_reg() -> Re {
    char('w') | char('W') | string_to_rexp("week") | string_to_rexp("Week")
}

pub fn month_time_reg() -> Re {
    string_to_rexp("month") | string_to_rexp("Month")
}

pub fn year_time_reg() -> Re {
    char('y') | char('Y') | string_to_rexp("year") | string_to_rexp("Year")
}
pub fn operator_reg() -> Re {
    char('+') | char('-') | char('/') | char('*') | string_to_rexp("//") | char('!')
}
pub fn date_reg() -> Re {
    (number_reg()
        + opt(whitespace_reg())
        + string_to_rexp("//")
        + opt(whitespace_reg())
        + number_reg()
        + opt(whitespace_reg())
        + string_to_rexp("//")
        + opt(whitespace_reg())
        + number_reg())
}

pub fn l_paren_reg() -> Re {
    char('(') | char('{')
}
pub fn r_paren_reg() -> Re {
    char(')') | char('}')
}
pub fn prog_reg() -> Re {
    star(
        recd("h", hour_reg())
            | recd("m", minute_reg())
            | recd("s", second_reg())
            | recd("n", number_reg())
            | recd("op", operator_reg())
            | recd("w", whitespace_reg())
            | recd("week", week_reg())
            | recd("day", day_reg())
            | recd("month", month_time_reg())
            | recd("date", date_reg())
            | recd("year", year_time_reg())
            | recd("l_paren", l_paren_reg())
            | recd("r_paren", r_paren_reg()),
    )
}

pub fn map_to_tokens(s: &(String, String)) -> Option<Token> {
    match s {
        (s1, s2) => match (s1.as_str(), s2) {
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
            _ => None,
        },
        _ => unreachable!(),
    }
}

pub fn tokenise(s: &str) -> Vec<Token> {
    lexing_simp(&prog_reg(), &s)
        .iter()
        .filter_map(|x| map_to_tokens(x))
        .collect()
}
