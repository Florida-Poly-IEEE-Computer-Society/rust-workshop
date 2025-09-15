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
