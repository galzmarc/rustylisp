use std::io;

use rustylisp::*;

fn main() {
    let env = &mut standard_env();
    loop {
        let mut expr = String::new();
        io::stdin()
            .read_line(&mut expr)
            .expect("Failed to read line");

        match parse_eval(expr, env) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("Error: {}", e),
        };
    }
}
