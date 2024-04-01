#[cfg(test)]
use crate::{ lexer::tokeniser::tokenise, parsers::main_parser::parse_all };
#[test]
fn test_parser() {
    //Test regular assignments, throw in some whitespace for fun
    let exp_1 = parse_all(tokenise("number
    
     = 3"));
    assert_eq!(format!("{:?}", exp_1), "(number = 3)");
    let exp_2 = parse_all(tokenise("number                  
                      = 3 + 4"));
    assert_eq!(format!("{:?}", exp_2), "(number = (3 + 4))");
    let exp_3 = parse_all(tokenise("number = 3 + 4 * 5"));
    assert_eq!(format!("{:?}", exp_3), "(number = (3 + (4 * 5)))");
    let exp_4 = parse_all(tokenise("number = 3 / 3 + 4 * 5"));
    assert_eq!(format!("{:?}", exp_4), "(number = ((3 / 3) + (4 * 5)))");

    // Test the parentheses as well
    let exp_5 = parse_all(tokenise("number = 3 / (3 + 4) * 5"));
    assert_eq!(format!("{:?}", exp_5), "(number = ((3 / (3 + 4)) * 5))");
    let exp_6 = parse_all(tokenise("number = 3 / 3 + (4 * 5)"));
    assert_eq!(format!("{:?}", exp_6), "(number = ((3 / 3) + (4 * 5)))");
    let exp_7 = parse_all(tokenise("number = (3 / 3) + (4 * 5)"));
    assert_eq!(format!("{:?}", exp_7), "(number = ((3 / 3) + (4 * 5)))");
    let exp_8 = parse_all(tokenise("number = 3 / 3 + 4 * (5 - 2)"));
    assert_eq!(format!("{:?}", exp_8), "(number = ((3 / 3) + (4 * (5 - 2))))");

    //Test conditional expressions
    let exp_9 = parse_all(tokenise("number = 3 == 3 ? 4 : 5"));
    assert_eq!(format!("{:?}", exp_9), "(number = if (3 == 3) then (4) else (5))");
    let exp_10 = parse_all(tokenise("number = 3 == 2 + 1 ? 4 - 2 : 5 * 6"));
    assert_eq!(
        format!("{:?}", exp_10),
        "(number = if (3 == (2 + 1)) then ((4 - 2)) else ((5 * 6)))"
    );
    let exp_11 = parse_all(tokenise("number = 3 * 6
     == 2 + 1 * 3 ? (4 - 2) * 6 : 
     5 * 6"));
    assert_eq!(
        format!("{:?}", exp_11),
        "(number = if ((3 * 6) == (2 + (1 * 3))) then (((4 - 2) * 6)) else ((5 * 6)))"
    );

    // Test function calls

    let exp_12 = parse_all(tokenise("function(one, two) = one"));
    assert_eq!(format!("{:?}", exp_12), "(function([one, two]) = one)");
    let exp_13 = parse_all(tokenise("function(one, two) = one * two"));
    assert_eq!(format!("{:?}", exp_13), "(function([one, two]) = (one * two))");
    let exp_12 = parse_all(tokenise("function(one, two) = 1"));
    assert_eq!(format!("{:?}", exp_12), "(function([one, two]) = 1)");
}
