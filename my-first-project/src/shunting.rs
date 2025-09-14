use crate::shared::*;

pub fn shunting_algo(tokens: Vec<Token>) -> OutputQueue {
    let mut output_queue = OutputQueue::new();
    let mut operator_stack = OperatorStack::new();
    for i in tokens {
        match i {
            Token::Whole(_) => output_queue.queue(i),
            Token::Floating(_, _) => output_queue.queue(i),
            Token::Operator(Operator::CloseParen) => {
                while !matches!(operator_stack.last(), Some(Operator::OpenParen)) {
                    output_queue.queue(Token::Operator(operator_stack.pop().unwrap()));
                }
                operator_stack.pop().unwrap();
            }
            Token::Operator(Operator::OpenParen) => {
                operator_stack.push(Operator::OpenParen);
            }
            Token::Operator(operator) => {
                while !matches!(operator_stack.last(), Some(Operator::OpenParen))
                    && if let Some(o2) = operator_stack.last() {
                        *o2 >= operator
                    } else {
                        false
                    }
                {
                    output_queue.queue(Token::Operator(operator_stack.pop().unwrap()));
                }
                operator_stack.push(operator);
            }
        }
    }
    // Still have not added case for open paren
    while let Some(top) = operator_stack.pop() {
        assert!(!matches!(top, Operator::OpenParen));
        output_queue.queue(Token::Operator(top));
    }
    output_queue
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shunting_algo_1() {
        // Ensure that (1 + 2) * ((3 / 2) + 91) is handled correctly
        // Should create postfix of 1 2 + 3 2 / 91 + *
        let input_tokens = vec![
            Token::Operator(Operator::OpenParen),
            Token::Whole(1.0),
            Token::Operator(Operator::Add),
            Token::Whole(2.0),
            Token::Operator(Operator::CloseParen),
            Token::Operator(Operator::Mul),
            Token::Operator(Operator::OpenParen),
            Token::Operator(Operator::OpenParen),
            Token::Whole(3.0),
            Token::Operator(Operator::Div),
            Token::Whole(2.0),
            Token::Operator(Operator::CloseParen),
            Token::Operator(Operator::Add),
            Token::Whole(91.0),
            Token::Operator(Operator::CloseParen),
        ];
        let mut output_queue = shunting_algo(input_tokens);
        test_number(1.0, output_queue.dequeue());
        test_number(2.0, output_queue.dequeue());
        assert!(matches!(
            output_queue.dequeue(),
            Some(Token::Operator(Operator::Add))
        ));
        test_number(3.0, output_queue.dequeue());
        test_number(2.0, output_queue.dequeue());
        assert!(matches!(
            output_queue.dequeue(),
            Some(Token::Operator(Operator::Div))
        ));
        test_number(91.0, output_queue.dequeue());
        assert!(matches!(
            output_queue.dequeue(),
            Some(Token::Operator(Operator::Add))
        ));
        assert!(matches!(
            output_queue.dequeue(),
            Some(Token::Operator(Operator::Mul))
        ));
    }

    fn test_number(real_val: f64, queued_value: Option<Token>) {
        let grabbed_float = match queued_value {
            Some(Token::Whole(float)) => float,
            Some(Token::Floating(_, float)) => float,
            Some(Token::Operator(_)) => {
                panic!("Value was an operator, looking for value {real_val}")
            }
            None => panic!("Value should not be empty"),
        };
        assert_eq!(real_val, grabbed_float);
    }
}
