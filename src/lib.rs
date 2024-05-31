use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Symbol(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    Atom(Atom),
    List(Vec<Exp>),
    Func(fn(&[Exp]) -> Exp),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    data: HashMap<String, Exp>,
}

impl Env {
    fn new() -> Self {
        Env {
            data: HashMap::new(),
        }
    }
    fn insert(&mut self, k: String, v: Exp) {
        self.data.insert(k, v);
    }
}

// Takes as input a string of characters; it adds spaces around each parenthesis,
// and then calls split to get a list of tokens
pub fn tokenize(str: String) -> Vec<String> {
    let result = str
        .replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|i| i.to_string())
        .collect();
    result
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Exp {
    // Read an expression from a sequence of tokens
    if tokens.is_empty() {
        panic!("Unexpected EOF.");
    }
    let token = tokens.remove(0);
    if token == "(" {
        let mut list: Vec<Exp> = Vec::new();
        while tokens[0] != ")" {
            list.push(read_from_tokens(tokens));
        }
        tokens.remove(0); // pop off ')'
        Exp::List(list)
    } else if token == ")" {
        panic!("Unexpected ')'.");
    } else {
        Exp::Atom(atom(token))
    }
}

fn atom(token: String) -> Atom {
    // Numbers become numbers; every other token is a symbol
    match token.parse::<f64>() {
        Ok(num) => Atom::Number(num),
        Err(_) => Atom::Symbol(token),
    }
}

pub fn parse(input: String) -> Exp {
    // Read a Scheme expression from a string
    read_from_tokens(&mut tokenize(input))
}

pub fn standard_env() -> Env {
    // An environment with some Scheme standard procedures
    let mut env = Env::new();
    env.insert("+".to_string(), Exp::Func(|args: &[Exp]| {
        add(args)
    }));
    env.insert("-".to_string(), Exp::Func(|args: &[Exp]| {
        subtract(args)
    }));
    env
}

fn add(args: &[Exp]) -> Exp {
    let sum = args.iter().fold(0.0, |acc, arg| {
        if let Exp::Atom(Atom::Number(num)) = arg {
            acc + num
        } else {
            panic!("Expected a number")
        }
    });
    Exp::Atom(Atom::Number(sum))
}

fn subtract(args: &[Exp]) -> Exp {
    let first = if let Some(Exp::Atom(Atom::Number(n))) = args.iter().next() {
        *n
    } else {
        panic!("Expected a number")
    };
    let result = args.iter().fold(first, |acc, arg| {
        if let Exp::Atom(Atom::Number(num)) = arg {
            acc - num
        } else {
            panic!("Expected a number")
        }
    });
    Exp::Atom(Atom::Number(result))
}
