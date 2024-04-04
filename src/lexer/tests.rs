#[cfg(test)]
use crate::{
    lexer::{ lexer::lexing_simp, tokeniser::tokenise },
    regex::reg::{ char, recd, star },
    tokens::tokens::Token::*,
};

#[test]
fn test_lexer() {
    let reg_1 = recd("commas", star(char(',')));
    assert_eq!(
        vec![("commas".to_string(), ",,,".to_string())],
        lexing_simp(&reg_1, ",,,").unwrap()
    );
    assert_eq!(vec![("commas".to_string(), ",".to_string())], lexing_simp(&reg_1, ",").unwrap());
    assert_ne!(vec![("commas".to_string(), ",".to_string())], lexing_simp(&reg_1, ",,,").unwrap());

    let reg_2 = recd("fullstops", star(char('.')));
    assert_eq!(
        vec![("fullstops".to_string(), "...".to_string())],
        lexing_simp(&reg_2, "...").unwrap()
    );
    assert_eq!(vec![("fullstops".to_string(), ".".to_string())], lexing_simp(&reg_2, ".").unwrap());
    assert_ne!(
        vec![("fullstops".to_string(), ".".to_string())],
        lexing_simp(&reg_2, "...").unwrap()
    );

    let reg_3 = star(reg_1 | reg_2);
    assert_eq!(
        vec![("commas".to_string(), ",,,".to_string())],
        lexing_simp(&reg_3, ",,,").unwrap()
    );
    assert_eq!(
        vec![("fullstops".to_string(), "...".to_string())],
        lexing_simp(&reg_3, "...").unwrap()
    );
    assert_eq!(
        vec![
            ("fullstops".to_string(), "...".to_string()),
            ("commas".to_string(), ",,,".to_string())
        ],
        lexing_simp(&reg_3, "...,,,").unwrap()
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string())
        ],
        lexing_simp(&reg_3, ",,,...").unwrap()
    );
    assert_ne!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string())
        ],
        lexing_simp(&reg_3, "...,,,").unwrap()
    );

    let reg_4 = recd("ltorgt", char('<') | char('>'));
    assert_eq!(vec![("ltorgt".to_string(), ">".to_string())], lexing_simp(&reg_4, ">").unwrap());
    assert_eq!(vec![("ltorgt".to_string(), "<".to_string())], lexing_simp(&reg_4, "<").unwrap());
    assert_ne!(vec![("ltorgt".to_string(), ",".to_string())], lexing_simp(&reg_4, "<").unwrap());

    let reg_5 = star(reg_3 | reg_4);
    assert_eq!(
        vec![
            ("fullstops".to_string(), "...".to_string()),
            ("commas".to_string(), ",,,".to_string())
        ],
        lexing_simp(&reg_5, "...,,,").unwrap()
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string())
        ],
        lexing_simp(&reg_5, ",,,...").unwrap()
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string()),
            ("ltorgt".to_string(), ">".to_string())
        ],
        lexing_simp(&reg_5, ",,,...>").unwrap()
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string()),
            ("ltorgt".to_string(), "<".to_string())
        ],
        lexing_simp(&reg_5, ",,,...<").unwrap()
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("ltorgt".to_string(), "<".to_string()),
            ("fullstops".to_string(), "...".to_string()),
            ("commas".to_string(), ",,".to_string()),
            ("ltorgt".to_string(), ">".to_string())
        ],
        lexing_simp(&reg_5, ",,,<...,,>").unwrap()
    );
    assert_ne!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("ltorgt".to_string(), "<".to_string()),
            ("fullstops".to_string(), "...".to_string()),
            ("commas".to_string(), ",,".to_string()),
            ("ltorgt".to_string(), ">".to_string())
        ],
        lexing_simp(&reg_5, ",,,<..,,<").unwrap()
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,".to_string()),
            ("ltorgt".to_string(), ">".to_string()),
            ("fullstops".to_string(), "..".to_string()),
            ("commas".to_string(), ",,".to_string()),
            ("ltorgt".to_string(), "<".to_string())
        ],
        lexing_simp(&reg_5, ",,>..,,<").unwrap()
    );
}
#[test]
fn test_tokeniser() {
    let prog_1 = tokenise("1 + 2").unwrap();
    assert_eq!(prog_1, vec![Number(1), Operator(String::from("+")), Number(2)]);
}
