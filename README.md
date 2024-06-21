# Lisp Interpreter

This project is a functional Lisp interpreter written in Rust, specifically targeting the Scheme dialect. It features a REPL, parser, evaluator, and environment handling.

## Features

- **REPL**: A Read-Eval-Print Loop for interactive programming.
- **Parser**: Parses Scheme expressions into abstract syntax trees.
- **Evaluator**: Evaluates expressions according to Scheme semantics.
- **Environment Handling**: Manages variable scope and function definitions.

- **Numbers**: Supports integers and floating point numbers, with pi being defined in the standard environment.
- **Boolean Values**: Supports boolean values and operations.
- **Arithmetic Operations**: Supports basic arithmetic operations (+, -, *, /).
- **Comparison operations**: Supports comparison operators (>, <, =, >=, <=)
- **Variable Definitions**: Allows user-define variables
- **Function Definitions**: Allows user-defined functions with support for recursion.
- **Conditional statements**: Supports the evaluation of 'if' statements

## Usage

In the REPL, you can enter Lisp expressions and evaluate them. Here are some examples:

```sh
> (+ 1 1)
2
> (+ (* 3 5) (- 10 6))
19
> (define r 10)
10
> (* pi (* r r))
314.1592653589793
> (define (square x) (* x x))
square
> (square 5)
25.0
> (square r)
100.0
> (define (circle-area r) (* pi (* r r)))
circle-area
> (circle-area (+ 5 5))
314.1592653589793
> (if (< r 8) true false)
false
> (define fact (n) (if (<= n 1) 1 (* n (fact (- n 1)))))
fact
> (fact 5)
120
```


### Contributing

Contributions are welcome! Feel free to submit issues or pull requests to enhance the interpreter's functionality, improve performance, or fix bugs.

### License

This project is licensed under the MIT License - see the LICENSE file for details.

### Acknowledgments

This project was inspired by Peter Norvig's [Lispy](http://norvig.com/lispy.html) and Stepan Parunashvili's [Risp](https://stopa.io/post/222).
