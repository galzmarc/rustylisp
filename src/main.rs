use std::env;

use rustylisp::{eval, parse, standard_env};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Error: No input provided. Please provide a Lisp expression.");
        std::process::exit(1);
    }

    let env = standard_env();
    let input = args.join(" ");
    let parsed_exp = match parse(input) {
        Ok(exp) => exp,
        Err(e) => {
            eprintln!("Error during parsing: {}", e);
            std::process::exit(1);
        }
    };
    let result = match eval(parsed_exp, &env) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error during evaluation: {}", e);
            std::process::exit(1);
        },
    };
    println!("{:?}", result);
}
