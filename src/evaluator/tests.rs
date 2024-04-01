#[cfg(test)]
#[test]
fn test_evaluator() {
    use chrono::format::Numeric;

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
            )
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
            )
        )
    );
    assert!(result_2.is_ok());
    assert_eq!(format!("{:?}", result_2.unwrap()), "Number(3628800)");

    let result_3 = evaluate(
        parse_programme(
            tokenise(
                "
    fib(number) = number == 1 ? 1 : 
    number == 0 ? 0 :
     fib(number - 1) + fib(number  -2 );
    result = fib(10);
    result
    "
            )
        )
    );
    assert!(result_3.is_ok());
    assert_eq!(format!("{:?}", result_3.unwrap()), "Number(55)");
}
