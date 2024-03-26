#[cfg(test)]
use crate::{ lexer::lexer::lexing_simp, regex::reg::{ char, recd, star } };

#[test]
fn test_lexer() {
    let reg_1 = recd("commas", star(char(',')));
    assert_eq!(vec![("commas".to_string(), ",,,".to_string())], lexing_simp(&reg_1, ",,,"));
    assert_eq!(vec![("commas".to_string(), ",".to_string())], lexing_simp(&reg_1, ","));
    assert_ne!(vec![("commas".to_string(), ",".to_string())], lexing_simp(&reg_1, ",,,"));

    let reg_2 = recd("fullstops", star(char('.')));
    assert_eq!(vec![("fullstops".to_string(), "...".to_string())], lexing_simp(&reg_2, "..."));
    assert_eq!(vec![("fullstops".to_string(), ".".to_string())], lexing_simp(&reg_2, "."));
    assert_ne!(vec![("fullstops".to_string(), ".".to_string())], lexing_simp(&reg_2, "..."));

    let reg_3 = star(reg_1 | reg_2);
    assert_eq!(vec![("commas".to_string(), ",,,".to_string())], lexing_simp(&reg_3, ",,,"));
    assert_eq!(vec![("fullstops".to_string(), "...".to_string())], lexing_simp(&reg_3, "..."));
    assert_eq!(
        vec![
            ("fullstops".to_string(), "...".to_string()),
            ("commas".to_string(), ",,,".to_string())
        ],
        lexing_simp(&reg_3, "...,,,")
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string())
        ],
        lexing_simp(&reg_3, ",,,...")
    );
    assert_ne!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string())
        ],
        lexing_simp(&reg_3, "...,,,")
    );

    let reg_4 = recd("ltorgt", char('<') | char('>'));
    assert_eq!(vec![("ltorgt".to_string(), ">".to_string())], lexing_simp(&reg_4, ">"));
    assert_eq!(vec![("ltorgt".to_string(), "<".to_string())], lexing_simp(&reg_4, "<"));
    assert_ne!(vec![("ltorgt".to_string(), ",".to_string())], lexing_simp(&reg_4, "<"));

    let reg_5 = star(reg_3 | reg_4);
    assert_eq!(
        vec![
            ("fullstops".to_string(), "...".to_string()),
            ("commas".to_string(), ",,,".to_string())
        ],
        lexing_simp(&reg_5, "...,,,")
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string())
        ],
        lexing_simp(&reg_5, ",,,...")
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string()),
            ("ltorgt".to_string(), ">".to_string())
        ],
        lexing_simp(&reg_5, ",,,...>")
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("fullstops".to_string(), "...".to_string()),
            ("ltorgt".to_string(), "<".to_string())
        ],
        lexing_simp(&reg_5, ",,,...<")
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("ltorgt".to_string(), "<".to_string()),
            ("fullstops".to_string(), "...".to_string()),
            ("commas".to_string(), ",,".to_string()),
            ("ltorgt".to_string(), ">".to_string())
        ],
        lexing_simp(&reg_5, ",,,<...,,>")
    );
    assert_ne!(
        vec![
            ("commas".to_string(), ",,,".to_string()),
            ("ltorgt".to_string(), "<".to_string()),
            ("fullstops".to_string(), "...".to_string()),
            ("commas".to_string(), ",,".to_string()),
            ("ltorgt".to_string(), ">".to_string())
        ],
        lexing_simp(&reg_5, ",,,<..,,<")
    );
    assert_eq!(
        vec![
            ("commas".to_string(), ",,".to_string()),
            ("ltorgt".to_string(), ">".to_string()),
            ("fullstops".to_string(), "..".to_string()),
            ("commas".to_string(), ",,".to_string()),
            ("ltorgt".to_string(), "<".to_string())
        ],
        lexing_simp(&reg_5, ",,>..,,<")
    );
}
