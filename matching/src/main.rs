fn char_to_i32(c: char) -> i32 {
    // exhausts all possible values of param c
    match c {
        // can have a code block
        'a' => {
            let x = 1;
            x
        }
        // or can just be an expression followed by a comma
        'b' => 2,
        _ => 3, // _ is the default value
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

// Derive the Clone trait
// Allows us to clone the state enumeration
#[derive(Debug, Clone)]
enum State {
    Pennsylvania,
    Florida,
    Washington,
    Virginia,
    NewYork,
}

impl Coin {
    fn coin_to_value(&self) -> u32 {
        match self {
            // Remember, Self is saying a Coin in general
            // Not this particalar coin instance self
            Self::Penny => 1,       // for Penny variant
            Self::Nickel => 5,      // for Nickel variant
            Self::Dime => 10,       // for Dime variant
            Self::Quarter(_) => 25, // for Quarter variant
        }
    }

    fn quarter_state(&self) -> Option<State> {
        match self {
            // pattern matching used to borrow the variants data
            Coin::Quarter(state) => Some(state.clone()),
            _ => None,
        }
    }
}

fn main() {
    let c1 = 'a';
    let i1 = char_to_i32(c1);
    println!("{i1}");

    // our functions in action
    let my_coin = Coin::Quarter(State::NewYork);
    let value = my_coin.coin_to_value();
    println!("{value}");

    let my_state: Option<State> = my_coin.quarter_state();
    // if let Some(state) will pattern match to the
    // variant of my_state
    if let Some(state) = my_state {
        println!("The state of my quarter is {state:?}");
    } else {
        println!("The coin was not a quarter");
    }
}
