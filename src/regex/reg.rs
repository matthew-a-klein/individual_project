
use Re::*;
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

pub fn nullable(r: &Re) -> bool {
    match r {
        ZERO => false,
        ONE => true,
        CHAR(_) => false,
        OPT(_) => true,
        ALT(l, r) => nullable(&*l) || nullable(&*r),
        SEQ(r1, r2) => nullable(&*r1) && nullable(&*r2),
        STAR(_) => true,
        RECD(_, r) => nullable(r),
    }
}

pub fn der(c: char, re: &Re) -> Re {
    match re {
        ZERO => ZERO,
        ONE => ZERO,
        CHAR(d) => {
            if c == *d {
                ONE
            } else {
                ZERO
            }
        }
        OPT(r) => der(c, r),
        ALT(l, r) => ALT(Box::new(der(c, &*l)), Box::new(der(c, &*r))), // Wrapped with Box::new
        SEQ(r1, r2) => {
            if nullable(&*r1) {
                ALT(
                    Box::new(SEQ(Box::new(der(c, &*r1)), Box::new(*r2.clone()))),
                    Box::new(der(c, &*r2)),
                )
            } else {
                SEQ(Box::new(der(c, &*r1)), Box::new(*r2.clone()))
            }
        }

        STAR(r) => SEQ(Box::new(der(c, r)), Box::new(STAR(Box::new(*r.clone())))),
        RECD(s, r) => RECD(s.to_string(), Box::new(der(c, r))),
    }
}

fn simp(re: &Re) -> &Re {
    match re {
        ALT(l, r) => match (simp(l), simp(r)) {
            (ZERO, rs) => rs,
            (ls, ZERO) => ls,
            (ls, rs) => {
                if rs == ls {
                    ls
                } else {
                    re
                }
            }
        },
        SEQ(r1, r2) => match (simp(r1), simp(r2)) {
            (ZERO, _) => &ZERO,
            (_, ZERO) => &ZERO,
            (ONE, r2) => r2,
            (r1, ONE) => r1,
            (_, _) => re,
        },
        re => re,
    }
}

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

pub fn matcher(r: &Re, s: &str) -> bool {
    nullable(&ders(s, r))
}

pub fn char(c: char) -> Re {
    CHAR(c)
}

pub fn concat(re1: Re, re2: Re) -> Re {
    SEQ(Box::new(re1), Box::new(re2))
}
pub fn opt(r: Re) -> Re {
    OPT(Box::new(r))
}

pub fn alt(re1: Re, re2: Re) -> Re {
    ALT(Box::new(re1), Box::new(re2))
}

pub fn star(re: Re) -> Re {
    STAR(Box::new(re))
}

pub fn recd(s: &str, r: Re) -> Re {
    RECD(s.to_string(), Box::new(r))
}

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

pub fn char_list_to_rexp(s: &[char]) -> Re {
    match s {
        [] => ONE,
        [c] => char(*c),
        [c, rest @ ..] => SEQ(Box::new(char(*c)), Box::new(char_list_to_rexp(rest))),
    }
}

pub fn string_to_rexp(s: &str) -> Re {
    char_list_to_rexp(s.chars().collect::<Vec<char>>().as_slice())
}
