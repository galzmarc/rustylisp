#[derive(Debug, Clone)]
pub enum Exp {
  Symbol(String),
  Number(f64),
  List(Vec<Exp>),
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

pub fn parse(program: String) -> Exp {
    // Read a Scheme expression from a string
    read_from_tokens(&mut tokenize(program))
}

pub fn read_from_tokens(tokens: &mut Vec<String>) -> Exp {
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
        atom(token)
    }
}

pub fn atom(token: String) -> Exp {
    // Numbers become numbers; every other token is a symbol
    match token.parse() {
        Ok(v) => Exp::Number(v),
        Err(_) => Exp::Symbol(token),
    }
}
