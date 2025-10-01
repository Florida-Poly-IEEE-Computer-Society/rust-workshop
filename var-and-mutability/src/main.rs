fn main() {
    // Is going to fail, use mut keyword or shadowing
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
