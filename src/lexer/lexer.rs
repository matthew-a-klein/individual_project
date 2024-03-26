use crate::regex::reg::nullable;
use crate::regex::reg::Re::*;
use crate::regex::reg::*;
use Val::*;
#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Empty,
    Chr(char),
    Sequ(Box<Val>, Box<Val>),
    Left(Box<Val>),
    Right(Box<Val>),
    Stars(Vec<Val>),
    // Extended Regular Expressions
    Rec(String, Box<Val>),
}
fn flatten(v: &Val) -> String {
    match v {
        Empty => String::new(),
        Chr(c) => c.to_string(),
        Left(v) => flatten(&*v),
        Right(v) => flatten(&*v),
        Sequ(v1, v2) => flatten(&*v1) + &flatten(&*v2),
        Stars(v) =>
            v
                .iter()
                .map(|v| flatten(&v))
                .collect::<Vec<String>>()
                .join(""), // Handle other cases accordingly
        Rec(_, v) => flatten(v),
    }
}

pub fn env(v: Val) -> Vec<(String, String)> {
    match v {
        Empty => Vec::new(),
        Chr(_) => Vec::new(),
        Left(v) => env(*v),
        Right(v) => env(*v),
        Sequ(v1, v2) => {
            let mut result = env(*v1);
            result.extend(env(*v2));
            result
        }
        Stars(v) =>
            v
                .into_iter()
                .flat_map(|v| env(v))
                .collect(),
        Rec(r, v) => {
            let mut result: Vec<(String, String)> = Vec::new();
            result.push((r, flatten(&v)));
            result.extend(env(*v));
            result
        }
    }
}
fn mkeps(r: &Re) -> Val {
    match r {
        ZERO => panic!(),
        ONE => Empty,
        OPT(_) => Empty,
        CHAR(_) => panic!(),
        ALT(l, r) => {
            if nullable(l) { Left(Box::new(mkeps(l))) } else { Right(Box::new(mkeps(r))) }
        }
        SEQ(v1, v2) => Sequ(Box::new(mkeps(v1)), Box::new(mkeps(v2))),
        STAR(_) => Stars(Vec::new()),
        RECD(s, r) => Rec(s.to_string(), Box::new(mkeps(r))),
    }
}

pub fn inj(r: &Re, c: char, v: Val) -> Val {
    match (r, v) {
        (CHAR(_), Empty) => Chr(c),

        (ALT(r1, _), Left(v1)) => Left(Box::new(inj(r1, c, *v1))),
        (ALT(_, r2), Right(v2)) => Right(Box::new(inj(r2, c, *v2))),

        (SEQ(r1, _), Sequ(v1, v2)) => Sequ(Box::new(inj(r1, c, *v1)), v2),
        (SEQ(r1, _), Left(v)) =>
            match *v {
                Sequ(v1, v2) => Sequ(Box::new(inj(r1, c, *v1)), v2),
                _ => panic!(),
            }
        (SEQ(r1, r2), Right(v)) =>
            match *v {
                v => Sequ(Box::new(mkeps(&*r1)), Box::new(inj(&*r2, c, v))),
            }

        (STAR(r), Sequ(v1, v2)) =>
            match *v2 {
                Stars(vs) => {
                    let mut res = Vec::new();
                    res.push(inj(r, c, *v1));
                    res.extend(vs);
                    Stars(res)
                }
                _ => panic!(),
            }
        // case (RECD(x, r1), _) => Rec(x, inj(r1, c, v))
        (RECD(s, r), Rec(_, v)) => Rec(s.to_string(), Box::new(inj(r, c, *v))),
        (r, v) => {
            println!("{:?},   {:?}", r, v);
            panic!()
        }
    }
}

fn f_id() -> Box<dyn Fn(Val) -> Val> {
    Box::new(move |v| v)
}

fn f_right<'a, F: 'a>(f: F) -> Box<dyn (Fn(Val) -> Val) + 'a> where F: Fn(Val) -> Val {
    Box::new(move |v| Right(Box::new(f(v))))
}

fn f_left<'a, F: 'a>(f: F) -> Box<dyn (Fn(Val) -> Val) + 'a> where F: Fn(Val) -> Val {
    Box::new(move |v| Left(Box::new(f(v))))
}

fn f_alt<'a, F1: 'a, F2: 'a>(f1: F1, f2: F2) -> Box<dyn (Fn(Val) -> Val) + 'a>
    where F1: Fn(Val) -> Val, F2: Fn(Val) -> Val
{
    Box::new(move |v| {
        match v {
            Left(v) => Left(Box::new(f1(*v))),
            Right(v) => Right(Box::new(f2(*v))),
            _ => panic!(),
        }
    })
}

fn f_seq<'a, F1: 'a, F2: 'a>(f1: F1, f2: F2) -> Box<dyn (Fn(Val) -> Val) + 'a>
    where F1: Fn(Val) -> Val, F2: Fn(Val) -> Val
{
    Box::new(move |v| {
        match v {
            Sequ(v1, v2) => Sequ(Box::new(f1(*v1)), Box::new(f2(*v2))),
            _ => panic!(),
        }
    })
}

fn f_seq_empty_1<'a, F1: 'a, F2: 'a>(f1: F1, f2: F2) -> Box<dyn (Fn(Val) -> Val) + 'a>
    where F1: Fn(Val) -> Val, F2: Fn(Val) -> Val
{
    Box::new(move |v| {
        match v {
            v => Sequ(Box::new(f1(Empty)), Box::new(f2(v))),
        }
    })
}

fn f_seq_empty_2<'a, F1: 'a, F2: 'a>(f1: F1, f2: F2) -> Box<dyn (Fn(Val) -> Val) + 'a>
    where F1: Fn(Val) -> Val, F2: Fn(Val) -> Val
{
    Box::new(move |v| {
        match v {
            v => Sequ(Box::new(f1(v)), Box::new(f2(Empty))),
        }
    })
}

fn f_recd<'a, F: 'a>(f: F) -> Box<dyn (Fn(Val) -> Val) + 'a> where F: Fn(Val) -> Val {
    Box::new(move |v| {
        match v {
            Rec(x, v) => Rec(x, Box::new(f(*v))),
            _ => panic!(),
        }
    })
}

fn f_error() -> Box<dyn Fn(Val) -> Val> {
    Box::new(move |_v| panic!())
}

// Simplify function
fn simp(r: Re) -> (Re, impl Fn(Val) -> Val) {
    match r {
        ALT(r1, r2) => {
            let (r1s, f1s) = simp(*r1);
            let (r2s, f2s) = simp(*r2);
            match (&r1s, &r2s) {
                (ZERO, _) => (r2s, f_right(f2s)),
                (_, ZERO) => (r1s, f_left(f1s)),
                _ if r1s == r2s => (r1s, f_left(f1s)),
                _ => (ALT(Box::new(r1s), Box::new(r2s)), f_alt(f1s, f2s)),
            }
        }
        SEQ(r1, r2) => {
            let (r1s, f1s) = simp(*r1);
            let (r2s, f2s) = simp(*r2);
            match (&r1s, &r2s) {
                (ZERO, _) => (ZERO, f_error()),
                (_, ZERO) => (ZERO, f_error()),
                (ONE, _) => (r2s, f_seq_empty_1(f1s, f2s)),
                (_, ONE) => (r1s, f_seq_empty_2(f1s, f2s)),

                _ => (SEQ(Box::new(r1s), Box::new(r2s)), f_seq(f1s, f2s)),
            }
        }
        RECD(s, r) => {
            let (rs, fs) = simp(*r);
            (RECD(s, Box::new(rs)), f_recd(fs))
        }
        r => (r, f_id()),
    }
}

pub fn lex_simp(r: &Re, s: Vec<char>) -> Val {
    if let Some((c, cs)) = s.split_first() {
        let (r_simp, f_simp) = simp(der(*c, r));
        inj(r, *c, f_simp(lex_simp(&r_simp, cs.to_vec())))
    } else {
        if nullable(r) { mkeps(r) } else { panic!("lexing error") }
    }
}

pub fn lexing_simp(r: &Re, s: &str) -> Vec<(String, String)> {
    env(lex_simp(r, s.chars().collect()))
}
