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
