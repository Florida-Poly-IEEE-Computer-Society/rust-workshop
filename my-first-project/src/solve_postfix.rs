use crate::shared::*;

// Take some Output_Queue stack, assuming that it is in postfix notation
// And return a floating number
pub fn solve_postfix(output_queue: OutputQueue) -> f64 {
    // To solve in postfix, we will need a stack and logic to handle the difference cases
    let mut stack: Vec<f64> = Vec::new();
    // loop through each token in the queue
    for i in output_queue {
        match i {
            Token::Whole(val) => stack.push(val),
            Token::Floating(_, val) => stack.push(val),
            Token::Operator(operator_type) => {
                // The operation should go: x operation y
                let x = stack.pop().expect("Stack should not be empty");
                let y = stack.pop().expect("Stack should not be empty");
                match operator_type {
                    Operator::Add => stack.push(add_floats(x, y)),
                    Operator::Sub => stack.push(substract_floats(x, y)),
                    Operator::Mul => stack.push(multiply_floats(x, y)),
                    Operator::Div => stack.push(divide_floats(x, y)),
                    _ => panic!("Unexpected operator"),
                }
            }
        }
    }
    stack
        .pop()
        .expect("Calculation finished, should have succeeded")
}

// Project functions that students will need to work on in order to fix their calculator
fn add_floats(x: f64, y: f64) -> f64 {
    x + y
}

fn substract_floats(x: f64, y: f64) -> f64 {
    y - x
}

fn multiply_floats(x: f64, y: f64) -> f64 {
    x * y
}

fn divide_floats(x: f64, y: f64) -> f64 {
    y / x
}
