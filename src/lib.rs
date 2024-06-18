use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Symbol(String),
    Number(f64),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Symbol(s) => write!(f, "{}", s),
            Atom::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    Atom(Atom),
    List(Vec<Exp>),
    Func(fn(&[Exp]) -> Exp),
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exp::Atom(atom) => write!(f, "{}", atom),
            Exp::List(list) => {
                let formatted_list: Vec<String> = list.iter().map(|exp| format!("{}", exp)).collect();
                write!(f, "({})", formatted_list.join(" "))
            }
            Exp::Func(_) => write!(f, "<function>"),
        }
    }
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
    fn get(&self, k: &str) -> Option<&Exp> {
        self.data.get(k)
    }
    fn insert(&mut self, k: String, v: Exp) {
        self.data.insert(k, v);
    }
}

// Takes as input a string of characters; it adds spaces around each parenthesis,
// and then calls split to get a list of tokens
pub fn tokenize(exp: String) -> Vec<String> {
    exp.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Result<Exp, String> {
    // Read an expression from a sequence of tokens
    if tokens.is_empty() {
        return Err("Unexpected EOF.".to_string());
    }
    let token = tokens.remove(0);
    if token == "(" {
        let mut list: Vec<Exp> = Vec::new();
        while tokens[0] != ")" {
            list.push(read_from_tokens(tokens)?);
        }
        tokens.remove(0); // pop off ')'
        Ok(Exp::List(list))
    } else if token == ")" {
        panic!("Unexpected ')'.");
    } else {
        Ok(Exp::Atom(atom(token)))
    }
}

fn atom(token: String) -> Atom {
    // Numbers become numbers; every other token is a symbol
    match token.parse::<f64>() {
        Ok(num) => Atom::Number(num),
        Err(_) => Atom::Symbol(token),
    }
}

pub fn parse(input: String) -> Result<Exp, String> {
    // Read a Scheme expression from a string
    read_from_tokens(&mut tokenize(input))
}

pub fn standard_env() -> Env {
    // An environment with some Lisp standard procedures
    let mut env = Env::new();
    // Adding basic arithmetic operations
    env.insert("+".to_string(), Exp::Func(|args: &[Exp]| add(args)));
    env.insert("-".to_string(), Exp::Func(|args: &[Exp]| subtract(args)));
    env.insert("*".to_string(), Exp::Func(|args: &[Exp]| multiply(args)));
    env.insert("/".to_string(), Exp::Func(|args: &[Exp]| divide(args)));

    env
}

pub fn eval(exp: Exp, env: &Env) -> Result<Exp, String> {
    match exp {
        Exp::Atom(Atom::Symbol(s)) => {
            env.get(&s).cloned().ok_or_else(|| panic!("Undefined symbol: {}", s))
        },
        Exp::Atom(Atom::Number(_)) => Ok(exp),
        Exp::List(list) => {
            let first = &list[0];
            if let Exp::Atom(Atom::Symbol(ref s)) = first {
                if let Some(Exp::Func(f)) = env.get(s) {
                    let args = list[1..].iter()
                        .map(|x| eval(x.clone(), env))
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(f(&args))
                } else {
                    panic!("Undefined function: {}", s);
                }
            } else {
                panic!("Expected a symbol");
            }
        },
        Exp::Func(_) => Ok(exp),
    }
}

fn add(args: &[Exp]) -> Exp {
    let sum = args.iter().fold(0.0, |acc, arg| {
        if let Exp::Atom(Atom::Number(num)) = arg {
            acc + num
        } else {
            panic!("Expected a number");
        }
    });
    Exp::Atom(Atom::Number(sum))
  }
  
fn subtract(args: &[Exp]) -> Exp {
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
  
fn multiply(args: &[Exp]) -> Exp {
    let first = if let Some(Exp::Atom(Atom::Number(n))) = args.iter().next() {
        *n
    } else {
        panic!("Expected a number");
    };
    let product = args.iter().skip(1).fold(first, |acc, arg| {
        if let Exp::Atom(Atom::Number(num)) = arg {
            acc * num
        } else {
            panic!("Expected a number");
        }
    });
    Exp::Atom(Atom::Number(product))
  }
  
fn divide(args: &[Exp]) -> Exp {
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