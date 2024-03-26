#[cfg(test)]
use crate::{ lexer::tokeniser::tokenise, parsers::main_parser::parse_all };
#[test]
fn test_parser() {
    let exp_1 = parse_all(tokenise("number = 3"));
    assert_eq!(format!("{:?}", exp_1), "(number = 3)");
    let exp_2 = parse_all(tokenise("number = 3 + 4"));
    assert_eq!(format!("{:?}", exp_2), "(number = (3 + 4))");
    let exp_3 = parse_all(tokenise("number = 3 + 4 * 5"));
    assert_eq!(format!("{:?}", exp_3), "(number = (3 + (4 * 5)))");
    let exp_4 = parse_all(tokenise("number = 3 / 3 + 4 * 5"));
    assert_eq!(format!("{:?}", exp_4), "(number = ((3 / 3) + (4 * 5)))");
}
