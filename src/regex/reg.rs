// Regular expression definition and associated functions.

// Represents different types of regular expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum Re {
    ZERO,
    ONE,
    CHAR(char),
    OPT(Box<Re>),
    ALT(Box<Re>, Box<Re>),
    SEQ(Box<Re>, Box<Re>),
    STAR(Box<Re>),
    RECD(String, Box<Re>),
}

// Determines if a regular expression is nullable.
pub fn nullable(r: &Re) -> bool {
    match r {
        Re::ZERO => false,
        Re::ONE => true,
        Re::CHAR(_) => false,
        Re::OPT(_) => true,
        Re::ALT(l, r) => nullable(&*l) || nullable(&*r),
        Re::SEQ(r1, r2) => nullable(&*r1) && nullable(&*r2),
        Re::STAR(_) => true,
        Re::RECD(_, r) => nullable(r),
    }
}

// Computes the derivative of a regular expression with respect to a character.
pub fn der(c: char, re: &Re) -> Re {
    match re {
        Re::ZERO => Re::ZERO,
        Re::ONE => Re::ZERO,
        Re::CHAR(d) => if c == *d { Re::ONE } else { Re::ZERO }
        Re::OPT(r) => der(c, r),
        Re::ALT(l, r) => Re::ALT(Box::new(der(c, &*l)), Box::new(der(c, &*r))),
        Re::SEQ(r1, r2) => {
            if nullable(&*r1) {
                Re::ALT(
                    Box::new(Re::SEQ(Box::new(der(c, &*r1)), Box::new(*r2.clone()))),
                    Box::new(der(c, &*r2))
                )
            } else {
                Re::SEQ(Box::new(der(c, &*r1)), Box::new(*r2.clone()))
            }
        }
        Re::STAR(r) => Re::SEQ(Box::new(der(c, r)), Box::new(Re::STAR(Box::new(*r.clone())))),
        Re::RECD(s, r) => Re::RECD(s.to_string(), Box::new(der(c, r))),
    }
}

// Simplifies a regular expression.
fn simp(re: &Re) -> &Re {
    match re {
        Re::ALT(l, r) =>
            match (simp(l), simp(r)) {
                (Re::ZERO, rs) => rs,
                (ls, Re::ZERO) => ls,
                (ls, rs) => if rs == ls { ls } else { re }
            }
        Re::SEQ(r1, r2) =>
            match (simp(r1), simp(r2)) {
                (Re::ZERO, _) => &Re::ZERO,
                (_, Re::ZERO) => &Re::ZERO,
                (Re::ONE, r2) => r2,
                (r1, Re::ONE) => r1,
                (_, _) => re,
            }
        _ => re,
    }
}

// Computes the derivative of a regular expression with respect to a string.
pub fn ders<'a>(s: &'a str, re: &'a Re) -> Re {
    match s.chars().next() {
        None => re.clone(),
        Some(c) => {
            let derived = der(c, re);
            let simplified = simp(&derived);
            ders(&s[1..], simplified)
        }
    }
}

// Determines if a given string matches a regular expression.
pub fn matcher(r: &Re, s: &str) -> bool {
    nullable(&ders(s, r))
}

// Creates a character regular expression.
pub fn char(c: char) -> Re {
    Re::CHAR(c)
}

// Concatenates two regular expressions.
pub fn concat(re1: Re, re2: Re) -> Re {
    Re::SEQ(Box::new(re1), Box::new(re2))
}

// Creates an optional regular expression.
pub fn opt(r: Re) -> Re {
    Re::OPT(Box::new(r))
}

// Creates an alternation of two regular expressions.
pub fn alt(re1: Re, re2: Re) -> Re {
    Re::ALT(Box::new(re1), Box::new(re2))
}

// Creates a Kleene star regular expression.
pub fn star(re: Re) -> Re {
    Re::STAR(Box::new(re))
}

// Creates a record regular expression.
pub fn recd(s: &str, r: Re) -> Re {
    Re::RECD(s.to_string(), Box::new(r))
}

// Implementations for operators for ease of use.
impl std::ops::Add for Re {
    type Output = Re;

    fn add(self, rhs: Re) -> Re {
        concat(self, rhs)
    }
}

impl std::ops::BitOr for Re {
    type Output = Re;

    fn bitor(self, rhs: Re) -> Re {
        alt(self, rhs)
    }
}

// Converts a list of characters into a regular expression.
pub fn char_list_to_rexp(s: &[char]) -> Re {
    match s {
        [] => Re::ONE,
        [c] => char(*c),
        [c, rest @ ..] => concat(char(*c), char_list_to_rexp(rest)),
    }
}

// Converts a string into a regular expression.
pub fn string_to_rexp(s: &str) -> Re {
    char_list_to_rexp(s.chars().collect::<Vec<char>>().as_slice())
}
