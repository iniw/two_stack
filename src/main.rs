use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

mod lex;
use lex::{Lexer, Operator, Token};

fn main() {
    let Some(filename) = env::args().nth(1) else {
        println!("usage: cargo run INPUT_FILE");
        return;
    };

    let Ok(file) = File::open(&filename) else {
        println!("Input file \"{filename}\" should exist and be readable.");
        return;
    };

    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("Failed to read line: \"{err}\"");
                continue;
            }
        };

        let (tokens, errors) = Lexer::new(&line).lex();

        if !errors.is_empty() {
            eprintln!("Lexing Errors:");
            for error in errors {
                eprintln!("  - {error}");
            }
            continue;
        }

        let mut value_stack = Vec::new();
        let mut operator_stack = Vec::new();

        // 1. While there are still items to read
        // 2. Get the next item
        for token in tokens {
            // 3. If the item is:
            match token {
                // A number:
                Token::Number(n) => {
                    // push it onto the value stack.
                    value_stack.push(n)
                }

                // A left parenthesis:
                Token::Operator(Operator::LeftParenthesis) => {
                    // push it onto the operator stack.
                    operator_stack.push(Operator::LeftParenthesis)
                }

                // A right parenthesis:
                Token::Operator(Operator::RightParenthesis) => {
                    // 1. While the top of the operator stack is not a left parenthesis
                    loop {
                        match operator_stack.last().copied() {
                            Some(Operator::LeftParenthesis) => break,
                            Some(op) => {
                                // 1. Pop the operator from the operator stack.
                                operator_stack.pop();

                                // 2. Pop the value stack twice, getting two operands.
                                let (left_operand, right_operand) =
                                    pop_value_stack_twice(&mut value_stack);

                                // 3. Apply the operator to the operands, in the correct order.
                                let result =
                                    apply_operator_to_operands(left_operand, op, right_operand);

                                // 4. Push the result onto the value stack.
                                value_stack.push(result);
                            }
                            None => unreachable!(),
                        }
                    }

                    // 2. Pop the left parenthesis from the operator stack
                    assert_eq!(operator_stack.pop(), Some(Operator::LeftParenthesis));
                }

                // - An operator op:
                Token::Operator(op) => {
                    // 1. While the operator stack is not empty,
                    while let Some(last_op) = operator_stack.last().copied() {
                        // and the top of the operator stack has the same or greater precedence as op
                        if last_op.precedence() >= op.precedence() {
                            // 1. Pop the operator from the operator stack.
                            let op = operator_stack
                                .pop()
                                .expect("We've checked for the existence of the last operator");

                            // 2. Pop the value stack twice, getting two operands.
                            let (left_operand, right_operand) =
                                pop_value_stack_twice(&mut value_stack);

                            // 3. Apply the operator to the operands, in the correct order.
                            let result =
                                apply_operator_to_operands(left_operand, op, right_operand);

                            // 4. Push the result onto the value stack.
                            value_stack.push(result);
                        } else {
                            break;
                        }
                    }

                    // 2. Push op onto the operator stack.
                    operator_stack.push(op);
                }
            }
        }

        // 2. While the operator stack is not empty,
        // 1. Pop the operator from the operator stack.
        for op in operator_stack.into_iter().rev() {
            // 2. Pop the value stack twice, getting two operands.
            let (left_operand, right_operand) = pop_value_stack_twice(&mut value_stack);

            // 3. Apply the operator to the operands, in the correct order.
            let result = apply_operator_to_operands(left_operand, op, right_operand);

            // 4. Push the result onto the value stack.
            value_stack.push(result);
        }

        if let Some(result) = value_stack.pop() {
            assert!(value_stack.is_empty());
            println!("{line} = {result}");
        }
    }
}

fn pop_value_stack_twice(value_stack: &mut Vec<f64>) -> (f64, f64) {
    let right_operand = value_stack
        .pop()
        .expect("At this point there should be at least two values in the value stack");
    let left_operand = value_stack
        .pop()
        .expect("At this point there should be at least two values in the value stack");

    (left_operand, right_operand)
}

fn apply_operator_to_operands(left_operand: f64, op: Operator, right_operand: f64) -> f64 {
    match op {
        Operator::Add => left_operand + right_operand,
        Operator::Sub => left_operand - right_operand,
        Operator::Mul => left_operand * right_operand,
        Operator::Div => left_operand / right_operand,
        _ => unreachable!(),
    }
}
