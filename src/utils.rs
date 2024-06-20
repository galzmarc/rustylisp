use crate::*;

pub fn add(args: &[Exp]) -> Exp {
    let sum = args.iter().fold(0.0, |acc, arg| {
        if let Exp::Atom(Atom::Number(num)) = arg {
            acc + num
        } else {
            panic!("Expected a number");
        }
    });
    Exp::Atom(Atom::Number(sum))
}

pub fn subtract(args: &[Exp]) -> Exp {
    let first = if let Some(Exp::Atom(Atom::Number(n))) = args.iter().next() {
        *n
    } else {
        panic!("Expected a number");
    };
    let result = args.iter().skip(1).fold(first, |acc, arg| {
        if let Exp::Atom(Atom::Number(num)) = arg {
            acc - num
        } else {
            panic!("Expected a number");
        }
    });
    Exp::Atom(Atom::Number(result))
}

pub fn multiply(args: &[Exp]) -> Exp {
    let product = args.iter().fold(1.0, |acc, arg| {
        if let Exp::Atom(Atom::Number(num)) = arg {
            acc * num
        } else {
            panic!("Expected a number");
        }
    });
    Exp::Atom(Atom::Number(product))
}

pub fn divide(args: &[Exp]) -> Exp {
    let first = if let Some(Exp::Atom(Atom::Number(n))) = args.iter().next() {
        *n
    } else {
        panic!("Expected a number");
    };
    let quotient = args.iter().skip(1).fold(first, |acc, arg| {
        if let Exp::Atom(Atom::Number(num)) = arg {
            if *num == 0.0 {
                panic!("Cannot divide by zero")
            }
            acc / num
        } else {
            panic!("Expected a number");
        }
    });
    Exp::Atom(Atom::Number(quotient))
}

pub fn compare(args: &[Exp], op: &str) -> Exp {
    if args.len() != 2 {
        panic!("Comparison operators require exactly two arguments");
    }

    let a = if let Exp::Atom(Atom::Number(n)) = args[0] {
        n
    } else {
        panic!("Expected a number");
    };

    let b = if let Exp::Atom(Atom::Number(n)) = args[1] {
        n
    } else {
        panic!("Expected a number");
    };

    let result = match op {
        "=" => a == b,
        ">" => a > b,
        "<" => a < b,
        ">=" => a >= b,
        "<=" => a <= b,
        _ => panic!("Unknown operator"),
    };

    Exp::Bool(result)
}
