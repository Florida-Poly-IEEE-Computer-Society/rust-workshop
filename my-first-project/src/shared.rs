use core::fmt;
use std::collections::VecDeque;

// Enumeration for the different token types that we are expecting
// to push on to the stack
#[derive(Debug, Clone)]
pub enum Token {
    Whole(f64),
    Floating(u64, f64),
    Operator(Operator),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    OpenParen,
    CloseParen,
}

// OperatorStack is a type alias for a Vector of Tokens
pub type OperatorStack = Vec<Operator>;

impl Operator {
    // Maps operators to u8 for comparison
    // Used to compare order of operation
    fn operator_order(&self) -> u8 {
        match self {
            Operator::Add => 1,
            Operator::Sub => 1,
            Operator::Mul => 2,
            Operator::Div => 2,
            Operator::CloseParen => 4,
            Operator::OpenParen => 4,
        }
    }
}

// Trait used to compare operators with == and !=
impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.operator_order() == other.operator_order()
    }
}

// Trait used to compare order with <, >, >=, and <=
impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.operator_order() > other.operator_order() {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

// OutputQueue used in the Shunting Yard Algorithm
#[derive(Debug, Clone)]
pub struct OutputQueue {
    data: VecDeque<Token>,
}

// Implementation of methods for OutputQueue struct
impl OutputQueue {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }
    pub fn dequeue(&mut self) -> Option<Token> {
        // Dequeue item at the front of the queue
        self.data.pop_front()
    }
    pub fn queue(&mut self, val: Token) {
        // Queue iten to the end of the queue
        self.data.push_back(val);
    }
}

// Implementing the display trait for OutputQueue
// You only need to know that the trait std::fmt::Display is
// used for macros like println!("") and to have your struct
// usable for that macros, it needs to implement std::fmt::Display
impl std::fmt::Display for OutputQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Queue values {:?}", self.data)
    }
}

// Implement the iterator trait
// This will be useful for calculating from postfix notation
impl std::iter::Iterator for OutputQueue {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.dequeue()
    }
}

// Unit testing section for rust
// Think of this as an integrated testing module to ensure code quality
#[cfg(test)]
mod test {
    use super::*;

    // Ensure that output_queue functions as expected
    #[test]
    fn test_output_queue() {
        let mut output_queue = OutputQueue::new();

        output_queue.queue(Token::Whole(56.0));
        output_queue.queue(Token::Operator(Operator::Mul));
        output_queue.queue(Token::Whole(67.0));
        output_queue.queue(Token::Operator(Operator::OpenParen));
        output_queue.queue(Token::Floating(1, 64.5));
        output_queue.queue(Token::Operator(Operator::Div));
        output_queue.queue(Token::Floating(1, 712.2));
        output_queue.queue(Token::Operator(Operator::CloseParen));

        // Ensure that the first item queued is the first out

        let first_queued = output_queue.dequeue();
        if let Some(Token::Whole(val)) = first_queued {
            assert_eq!(val, 56.0);
        } else {
            panic!("Queued the wrong value");
        }

        // Ensure that a queue is exhausted when looped through
        for i in &mut output_queue {
            println!("{i:?}");
        }

        assert_eq!(0, output_queue.data.len());
    }

    // Ensure operator comparison is working correctly, such that
    #[test]
    fn test_operator_order() {
        assert!(Operator::Mul == Operator::Div);
        assert!(Operator::CloseParen == Operator::OpenParen);
        assert!(Operator::Add == Operator::Sub);
        assert!(Operator::Add < Operator::Mul);
        assert!(Operator::Sub < Operator::Mul);
        assert!(Operator::Add < Operator::Div);
        assert!(Operator::Sub < Operator::Div);
        assert!(Operator::Mul > Operator::Add);
        assert!(Operator::Mul > Operator::Sub);
        assert!(Operator::Div > Operator::Sub);
        assert!(Operator::Div > Operator::Add);
        assert!(Operator::Mul < Operator::OpenParen);
        assert!(Operator::Div < Operator::OpenParen);
        assert!(Operator::Mul < Operator::CloseParen);
        assert!(Operator::Div < Operator::CloseParen);
        assert!(Operator::OpenParen > Operator::Mul);
        assert!(Operator::OpenParen > Operator::Div);
        assert!(Operator::CloseParen > Operator::Mul);
        assert!(Operator::CloseParen > Operator::Div);
    }
}
