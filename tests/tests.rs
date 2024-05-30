use rustylisp::tokenize;

#[test]
fn test_tokenize() {
    let input = String::from("(begin (define r 10) (* pi (* r r)))");
    let expected = vec![
        "(", "begin", "(", "define", "r", "10", ")", "(", "*", "pi", "(", "*", "r", "r", ")", ")",
        ")",
    ];
    assert_eq!(expected, tokenize(input));
}
