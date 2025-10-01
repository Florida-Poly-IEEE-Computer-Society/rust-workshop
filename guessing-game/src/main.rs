use std::io; // to get user input, we need to bring std::io into scope

// main entry into programs
fn main() {
    // macro that print values
    println!("Guess the number!");

    println!("Please input your guess.");

    // use a variable to store the users input
    // use the "let" statement to create a variable
    // let guess = String::new(); -> immutable (can not be changed)
    // let mut guess = String::new(); -> mutable (can be changed)
    let mut guess = String::new();

    // call the stdin function from the io module to handle user input
    // represents a structure that will handle the users input
    // method that will store the value of input into variable guess
    // & mut passes guess as a mutable reference
    // read_line return the enumeration Result
    // .expect will fail when the enumeration is a certain variant
    io::stdin().read_line(&mut guess).expect("Failed to read line"); 

    // prints the value of the variable guess
    println!("You guessed: {guess}");
}
