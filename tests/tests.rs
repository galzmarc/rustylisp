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
    let env = standard_env();
    let input = String::from("(+ 1 2 3)");
    let parsed_exp = parse(input).expect("Failed to parse input");
    let result = eval(parsed_exp, &env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(6.0)));
}

#[test]
fn test_eval_subtract() {
    let env = standard_env();
    let input = String::from("(- 10 2 3)");
    let parsed_exp = parse(input).expect("Failed to parse input");
    let result = eval(parsed_exp, &env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(5.0)));
}

#[test]
fn test_eval_multiply() {
    let env = standard_env();
    let input = String::from("(* 1 2 3)");
    let parsed_exp = parse(input).expect("Failed to parse input");
    let result = eval(parsed_exp, &env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(6.0)));
}

#[test]
fn test_eval_divide() {
    let env = standard_env();
    let input = String::from("(/ 24 6 2)");
    let parsed_exp = parse(input).expect("Failed to parse input");
    let result = eval(parsed_exp, &env).expect("Failed to evaluate expression");
    assert_eq!(result, Exp::Atom(Atom::Number(2.0)));
}