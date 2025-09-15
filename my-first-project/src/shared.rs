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
