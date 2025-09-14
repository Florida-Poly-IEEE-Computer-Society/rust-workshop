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
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power() {
        // test for exp 2
        let base = 10.0;
        let exp = 2;
        let answer = 100.0;
        assert_eq!(answer, power(base, exp));

        // test for exp of 0
        let base = 10.0;
        let exp = 0;
        let answer = 1.0;
        assert_eq!(answer, power(base, exp));
    }

    #[test]
    fn test_parse() {
        let test_string = "532.21   +  21.2314 (32 /2)*3-21";
        let mut output_vec = parse(test_string);

        // should be 21.0
        let last_val = output_vec.pop();
        if let Some(Token::Whole(val)) = last_val {
            assert_eq!(21.0, val);
        } else {
            panic!("This value should be 21.0. Instead it is {last_val:?}");
        }

        // should be subtraction
        let last_val = output_vec.pop();
        if let Some(Token::Operator(Operator::Sub)) = last_val {
        } else {
            panic!("This token should be subtraction. Instead it is {last_val:?}");
        }

        // should be 3.0
        let last_val = output_vec.pop();
        if let Some(Token::Whole(val)) = last_val {
            assert_eq!(3.0, val);
        } else {
            panic!("This value should be 3.0. Instead it is {last_val:?}");
        }

        // should be multiplication
        let last_val = output_vec.pop();
        if let Some(Token::Operator(Operator::Mul)) = last_val {
        } else {
            panic!("This token should be subtraction. Instead it is {last_val:?}");
        }

        // should be CloseParen
        let last_val = output_vec.pop();
        if let Some(Token::Operator(Operator::CloseParen)) = last_val {
        } else {
            panic!("This token should be subtraction. Instead it is {last_val:?}");
        }

        // should be 2.0
        let last_val = output_vec.pop();
        test_whole(2.0, last_val);

        // should be division
        let last_val = output_vec.pop();
        if let Some(Token::Operator(Operator::Div)) = last_val {
        } else {
            panic!("This token should be subtraction. Instead it is {last_val:?}");
        }

        // should be 32.0
        let last_val = output_vec.pop();
        if let Some(Token::Whole(val)) = last_val {
            assert_eq!(32.0, val);
        } else {
            panic!("This value should be 32.0. Instead it is {last_val:?}");
        }

        // should be openParen
        let last_val = output_vec.pop();
        if let Some(Token::Operator(Operator::OpenParen)) = last_val {
        } else {
            panic!("This token should be subtraction. Instead it is {last_val:?}");
        }

        // should be 21.2314
        let last_val = output_vec.pop();
        test_float(21.2314, 5, last_val);

        // should be Addition
        let last_val = output_vec.pop();
        if let Some(Token::Operator(Operator::Add)) = last_val {
        } else {
            panic!("This token should be subtraction. Instead it is {last_val:?}");
        }

        // should be 532.21
        let last_val = output_vec.pop();
        if let Some(Token::Floating(exp, val)) = last_val {
            assert_eq!(532.21, val);
            assert_eq!(3, exp);
        } else {
            panic!("This value should be 532.21. Instead it is {last_val:?}");
        }

        // output_vec should be empty
        if output_vec.pop().is_some() {
            panic!("output_vec should be empty");
        }
    }

    fn test_float(real_val: f64, prec: u64, queued_value: Option<Token>) {
        let (grabbed_prec, grabbed_float) = match queued_value {
            Some(Token::Floating(prec, float)) => (prec, float),
            Some(token) => panic!("Value was {token:?}, looking for a float token"),
            None => panic!("Value should not be empty"),
        };
        assert_eq!(real_val, grabbed_float);
        assert_eq!(prec, grabbed_prec);
    }

    fn test_whole(real_val: f64, queued_value: Option<Token>) {
        let whole = match queued_value {
            Some(Token::Whole(whole)) => whole,
            Some(token) => panic!("Value was {token:?}, looking for a float token"),
            None => panic!("Value should not be empty"),
        };
        assert_eq!(real_val, whole);
    }
}
