use rustylisp::*;

#[test]
fn test_tokenize() {
    let input = String::from("(begin (define r 10) (* pi (* r r)))");
    let expected = vec![
        "(", "begin", "(", "define", "r", "10", ")", "(", "*", "pi", "(", "*", "r", "r", ")", ")",
        ")",
    ];
    assert_eq!(expected, tokenize(input));
}

#[test]
fn test_parse() {
    let input = String::from("(begin (define r 10))");
    let expected = Exp::List(vec![
        Exp::Atom(Atom::Symbol("begin".to_string())),
        Exp::List(vec![
            Exp::Atom(Atom::Symbol("define".to_string())),
            Exp::Atom(Atom::Symbol("r".to_string())),
            Exp::Atom(Atom::Number(10.0)),
        ]),
    ]);
    assert_eq!(expected, parse(input).expect("Failed to parse input"));
}

#[test]
fn test_eval_add() {
    let env = &mut standard_env();
    let input = String::from("(+ 1 2 3)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(6.0)));
}

#[test]
fn test_eval_subtract() {
    let env = &mut standard_env();
    let input = String::from("(- 10 2 3)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(5.0)));
}

#[test]
fn test_eval_multiply() {
    let env = &mut standard_env();
    let input = String::from("(* 1 2 3)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(6.0)));
}

#[test]
fn test_eval_divide() {
    let env = &mut standard_env();
    let input = String::from("(/ 24 6 2)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(2.0)));
}

#[test]
fn test_boolean_literals() {
    let env = &mut standard_env();
    let input_true = String::from("true");
    let input_false = String::from("false");

    let result_true = parse_eval(input_true, env).expect("Failed to evaluate expression");
    assert_eq!(result_true, Exp::Bool(true));

    let result_false = parse_eval(input_false, env).expect("Failed to evaluate expression");
    assert_eq!(result_false, Exp::Bool(false));
}

#[test]
fn test_comparison_operators() {
    let env = &mut standard_env();

    let input = String::from("(= 1 1)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Bool(true));

    let input = String::from("(= 1 2)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Bool(false));

    let input = String::from("(> 2 1)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Bool(true));

    let input = String::from("(< 1 2)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Bool(true));

    let input = String::from("(>= 2 2)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Bool(true));

    let input = String::from("(<= 1 2)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Bool(true));
}

#[test]
fn test_define() {
    let env = &mut standard_env();
    let input = String::from("(define r 10)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(10.0)));
}

#[test]
fn test_define_function() {
    let env = &mut standard_env();
    let input = String::from("(define (square x) (* x x))");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Symbol("square".to_string())));

    let input = String::from("(square 5)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(25.0)));
}

#[test]
fn test_if_expression() {
    let env = &mut standard_env();
    let input = String::from("(if (= 1 1) 10 20)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(10.0)));

    let input = String::from("(if (= 1 2) 10 20)");
    let result = parse_eval(input, env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(20.0)));
}
