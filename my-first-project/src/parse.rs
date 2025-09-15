// Parsing section. Implement parsing functionality here
use std::panic;

use crate::shared::*;

// Create an algorithm that takes an
// infix string, and parse into a Queue of tokens
pub fn parse(infix_str: &str) -> Vec<Token> {
    let mut output_queue: Vec<Token> = Vec::new();
    // tells whether the previous value was part of a float or not
    for i in infix_str.chars() {
        match i {
            ' ' => continue,
            '+' => output_queue.push(Token::Operator(Operator::Add)),
            '-' => output_queue.push(Token::Operator(Operator::Sub)),
            '/' => output_queue.push(Token::Operator(Operator::Div)),
            '*' => output_queue.push(Token::Operator(Operator::Mul)),
            '(' => output_queue.push(Token::Operator(Operator::OpenParen)),
            ')' => output_queue.push(Token::Operator(Operator::CloseParen)),
            '.' => {
                if let Some(Token::Whole(val)) = output_queue.last() {
                    // Force block to own clone of val
                    let val = *val;
                    output_queue
                        .pop()
                        .expect("Last value should be able to be popped");
                    output_queue.push(Token::Floating(1, val));
                } else {
                    panic!("Could not handle decimal in its location");
                }
            }
            _ => {
                let digit = if let Some(digit) = i.to_digit(10) {
                    digit
                } else {
                    panic!("Could not parse char {i}");
                };
                match output_queue.last() {
                    Some(Token::Whole(val)) => {
                        // Force block to own clone of val
                        let val = *val;
                        output_queue
                            .pop()
                            .expect("Last value should be able to be popped");
                        output_queue.push(Token::Whole(val * 10.0 + f64::from(digit)));
                    }
                    Some(Token::Floating(prec, val)) => {
                        // Force block to own clone of val and prec
                        let prec = *prec;
                        let val = *val;
                        output_queue
                            .pop()
                            .expect("Last value should be able to be popped");
                        output_queue.push(Token::Floating(
                            prec + 1,
                            val + (f64::from(digit) / power(10.0, prec)),
                        ));
                    }
                    _ => output_queue.push(Token::Whole(f64::from(digit))),
                }
            }
        }
    }
    output_queue
}

// Returns float base to the power exp
fn power(base: f64, mut exp: u64) -> f64 {
    let mut operand = 1.0;
    while exp > 0 {
        exp -= 1;
        operand *= base;
    }
    operand
}
