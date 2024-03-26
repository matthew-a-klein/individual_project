#[cfg(test)]
use crate::regex::reg::{ char, matcher, opt, star };
#[test]
fn test_matcher() {
    // Test single character
    let reg1 = char('a');
    assert_eq!(matcher(&reg1, "a"), true);
    assert_eq!(matcher(&reg1, "aa"), false);
    assert_eq!(matcher(&reg1, ""), false);

    //Test star
    let reg2 = star(reg1);
    assert_eq!(matcher(&reg2, ""), true);
    assert_eq!(matcher(&reg2, "aaaa"), true);
    assert_eq!(matcher(&reg2, "a"), true);
    assert_eq!(matcher(&reg2, "ab"), false);
    assert_eq!(matcher(&reg2, "b"), false);

    //Test alternative
    let reg3 = char('b');
    let reg4 = reg2 | reg3;
    assert_eq!(matcher(&reg4, "b"), true);
    assert_eq!(matcher(&reg4, "bb"), false);
    assert_eq!(matcher(&reg4, "aaaa"), true);
    assert_eq!(matcher(&reg4, ""), true);
    assert_eq!(matcher(&reg4, "aaaab"), false);
    assert_eq!(matcher(&reg4, "c"), false);

    //Test sequence
    let reg6 = char('c') + char('d');
    assert_eq!(matcher(&reg6, "cd"), true);
    assert_eq!(matcher(&reg6, "dc"), false);
    assert_eq!(matcher(&reg6, "c"), false);
    assert_eq!(matcher(&reg6, "d"), false);

    //Test opt
    let reg7 = opt(char('e'));
    assert_eq!(matcher(&reg7, ""), true);
    assert_eq!(matcher(&reg7, "e"), true);
    assert_eq!(matcher(&reg7, "ee"), false);
    assert_eq!(matcher(&reg7, "d"), false);
    assert_eq!(matcher(&reg7, "eeee"), false);
}
