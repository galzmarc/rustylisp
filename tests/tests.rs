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
    assert_eq!(expected, parse(input));
}
