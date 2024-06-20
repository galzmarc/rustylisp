use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt;

use utils::{add, compare, divide, multiply, subtract};
mod utils;

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
    Bool(bool),
    Atom(Atom),
    List(Vec<Exp>),
    Func(fn(&[Exp]) -> Exp),
    FuncDef {
        params: Vec<Exp>,
        body: Vec<Exp>,
        env: Env,
    },
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exp::Bool(a) => write!(f, "{}", a),
            Exp::Atom(atom) => write!(f, "{}", atom),
            Exp::List(list) => {
                let formatted_list: Vec<String> =
                    list.iter().map(|exp| format!("{}", exp)).collect();
                write!(f, "({})", formatted_list.join(" "))
            }
            Exp::Func(_) => write!(f, "<function>"),
            Exp::FuncDef { .. } => write!(f, "<function>"),
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
    fn insert(&mut self, k: String, v: Exp) {
        self.data.insert(k, v);
    }
    pub fn get(&self, k: &str) -> Option<&Exp> {
        self.data.get(k)
    }
}

pub fn tokenize(exp: String) -> Vec<String> {
    // Takes as input a string of characters; it adds spaces around each parenthesis,
    // and then calls split to get a list of tokens
    exp.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Result<Exp, String> {
    // Read an expression from a sequence of tokens
    if tokens.is_empty() {
        return Err("No input provided. Please provide a Scheme expression.".to_string());
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
        return Err(format!("Unexpected ')'."));
    } else {
        Ok(atom(token))
    }
}

fn atom(token: String) -> Exp {
    match token.as_str() {
        "true" => Exp::Bool(true),
        "false" => Exp::Bool(false),
        // Numbers become numbers; every other token is a symbol
        _ => match token.parse::<f64>() {
            Ok(num) => Exp::Atom(Atom::Number(num)),
            Err(_) => Exp::Atom(Atom::Symbol(token)),
        },
    }
}

pub fn parse(input: String) -> Result<Exp, String> {
    // Read a Scheme expression from a string
    read_from_tokens(&mut tokenize(input))
}

pub fn parse_eval(input: String, env: &mut Env) -> Result<Exp, String> {
    let parsed_exp = match parse(input) {
        Ok(exp) => exp,
        Err(e) => {
            eprintln!("Error during parsing: {}", e);
            std::process::exit(1);
        }
    };
    let eval_exp = eval(parsed_exp, env)?;
    Ok(eval_exp)
}

pub fn standard_env() -> Env {
    // An environment with some Scheme standard procedures
    let mut env = Env::new();
    // Adding basic arithmetic operations
    env.insert("+".to_string(), Exp::Func(|args: &[Exp]| add(args)));
    env.insert("-".to_string(), Exp::Func(|args: &[Exp]| subtract(args)));
    env.insert("*".to_string(), Exp::Func(|args: &[Exp]| multiply(args)));
    env.insert("/".to_string(), Exp::Func(|args: &[Exp]| divide(args)));
    // Adding pi
    env.insert("pi".to_string(), Exp::Atom(Atom::Number(PI)));
    // Adding comparison operators
    env.insert(
        "=".to_string(),
        Exp::Func(|args: &[Exp]| compare(args, "=")),
    );
    env.insert(
        ">".to_string(),
        Exp::Func(|args: &[Exp]| compare(args, ">")),
    );
    env.insert(
        "<".to_string(),
        Exp::Func(|args: &[Exp]| compare(args, "<")),
    );
    env.insert(
        ">=".to_string(),
        Exp::Func(|args: &[Exp]| compare(args, ">=")),
    );
    env.insert(
        "<=".to_string(),
        Exp::Func(|args: &[Exp]| compare(args, "<=")),
    );

    env
}

fn eval(exp: Exp, env: &mut Env) -> Result<Exp, String> {
    match exp {
        Exp::Bool(_) => Ok(exp),
        Exp::Atom(Atom::Symbol(s)) => env
            .get(&s)
            .cloned()
            .ok_or_else(|| format!("Undefined symbol: {}", s)),
        Exp::Atom(Atom::Number(_)) => Ok(exp),
        Exp::List(list) => {
            let first = &list[0];
            if let Exp::Atom(Atom::Symbol(ref s)) = first {
                match s.as_str() {
                    "define" => eval_define(&list, env),
                    "if" => eval_if(&list, env),
                    _ => {
                        if let Some(exp) = env.get(s) {
                            match exp {
                                Exp::Func(f) => {
                                    // Clone the function to avoid borrowing `env` later
                                    let function = f.clone();
                                    let args: Result<Vec<Exp>, String> =
                                        list[1..].iter().map(|x| eval(x.clone(), env)).collect();
                                    Ok(function(&args?))
                                }
                                Exp::FuncDef {
                                    params,
                                    body,
                                    env: closure_env,
                                } => {
                                    // Clone `env` to avoid borrowing later
                                    let env_clone = &mut env.clone();
                                    let args: Result<Vec<Exp>, String> = list[1..]
                                        .iter()
                                        .map(|x| eval(x.clone(), env_clone))
                                        .collect();
                                    let mut local_env = closure_env.clone();
                                    for (param, arg) in params.iter().zip(args?) {
                                        if let Exp::Atom(Atom::Symbol(param_name)) = param {
                                            local_env.insert(param_name.clone(), arg);
                                        } else {
                                            return Err("Invalid parameter name".into());
                                        }
                                    }
                                    let mut result = Exp::Bool(false);
                                    for exp in body {
                                        result = eval(exp.clone(), &mut local_env)?;
                                    }
                                    Ok(result)
                                }
                                _ => Err(format!("Undefined function: {}", s)),
                            }
                        } else {
                            Err(format!("Undefined function: {}", s))
                        }
                    }
                }
            } else {
                Err("Expected a symbol".into())
            }
        }
        Exp::Func(_) => Ok(exp),
        Exp::FuncDef { .. } => Err("Unexpected function definition".into()),
    }
}

fn eval_define(list: &[Exp], env: &mut Env) -> Result<Exp, String> {
    if list.len() < 3 {
        return Err("'define' requires at least two arguments".into());
    }
    // Define a new function
    if let Exp::List(ref func) = list[1] {
        if let Exp::Atom(Atom::Symbol(ref func_name)) = func[0] {
            let params = func[1..].to_vec();
            let body = list[2..].to_vec();
            let lambda = Exp::FuncDef {
                params,
                body,
                env: env.clone(),
            };
            env.insert(func_name.clone(), lambda);
            return Ok(Exp::Atom(Atom::Symbol(func_name.clone())));
        } else {
            return Err("Invalid define syntax".into());
        }
    // Define a new variable
    } else if let Exp::Atom(Atom::Symbol(ref var_name)) = list[1] {
        let value = eval(list[2].clone(), env)?;
        env.insert(var_name.clone(), value.clone());
        return Ok(value);
    } else {
        return Err("Invalid define syntax".into());
    }
}

fn eval_if(list: &[Exp], env: &mut Env) -> Result<Exp, String> {
    if list.len() < 4 {
        return Err("'define' requires at least two arguments".into());
    }
    let condition = eval(list[1].clone(), env)?;
    match condition {
        Exp::Bool(true) => eval(list[2].clone(), env),
        Exp::Bool(false) => eval(list[3].clone(), env),
        _ => Err("Invalid condition in if expression".into()),
    }
}
