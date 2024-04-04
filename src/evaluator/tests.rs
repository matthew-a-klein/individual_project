#[cfg(test)]
#[test]
fn test_evaluator() {
    use crate::{
        evaluator::evaluator::evaluate,
        lexer::tokeniser::tokenise,
        parsers::main_parser::parse_programme,
    };

    let result_1 = evaluate(
        parse_programme(
            tokenise(
                "max(argone, argtwo) = argone < argtwo? argtwo: argone;
                add(argone, argtwo) = argone + argtwo;
                sum = add(max(4, 3), max(2, 1));
                sum"
            ).unwrap()
        )
    );
    assert!(result_1.is_ok());
    assert_eq!(format!("{:?}", result_1.unwrap()), "Number(6)");

    let result_2 = evaluate(
        parse_programme(
            tokenise(
                "
    fact(number) = number == 0 ? 1 : number * fact(number - 1);
    result = fact(10);
    result
    "
            ).unwrap()
        )
    );
    assert!(result_2.is_ok());
    assert_eq!(format!("{:?}", result_2.unwrap()), "Number(3628800)");

    let result_3 = evaluate(
        parse_programme(
            tokenise(
                "
    /*Test the fibonacci numbers*/
    fib(number) = number == 1 ? 1 : 
    number == 0 ? 0 :
    fib(number - 1) + fib(number - 2);
    result = fib(10);
    result
    "
            ).unwrap()
        )
    );
    assert!(result_3.is_ok());
    assert_eq!(format!("{:?}", result_3.unwrap()), "Number(55)");

    let result_4 = evaluate(
        parse_programme(
            tokenise(
                "
    /*Test the fibonacci numbers*/
    fib(number) = number == 1 ? 1 : 
    number == 0 ? 0 :
    fib(number - 1) + fib(number - 2);
    result = fib(14);
    result
    "
            ).unwrap()
        )
    );
    assert!(result_4.is_ok());
    assert_eq!(format!("{:?}", result_4.unwrap()), "Number(377)");

    let result_5 = evaluate(
        parse_programme(
            tokenise(
                "
    /*Test the triangle numbers recursively
    Causes stack overflow if pushed too much*/
    tri(number) = number == 1 ? 1 :
    tri(number - 1) + number;
    result = tri(100);
    result
    "
            ).unwrap()
        )
    );
    assert!(result_5.is_ok());
    assert_eq!(format!("{:?}", result_5.unwrap()), "Number(5050)");

    let result_6 = evaluate(
        parse_programme(
            tokenise(
                "
    /*Test the triangle numbers non arithmetically*/
    tri(number) = (number + 1) * number / 2;
    result = tri(1000);
    result
    "
            ).unwrap()
        )
    );
    assert!(result_6.is_ok());
    assert_eq!(format!("{:?}", result_6.unwrap()), "Number(500500)");

    let result_7 = evaluate(
        parse_programme(
            tokenise(
                "
    /*Test the triangle numbers recursively
    Causes stack overflow if pushed too much*/
    tri(number) = number == 1 ? 1 :
    tri(number - 1) + number;
    result = tri(100);
    result
    "
            ).unwrap()
        )
    );
    assert!(result_7.is_ok());
    assert_ne!(format!("{:?}", result_7.unwrap()), "Number(5051)");
}
