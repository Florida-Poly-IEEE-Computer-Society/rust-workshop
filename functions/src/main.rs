fn main() {
    let x = 5;
    let y = 10;
    let z = sum(x, y);
    println!("{z}");
}

fn my_first_function() {
    println!("Hello. I am your function. YIPPIE!!!");
}

fn function_with_parameters(x: u32) {
    println!("The number that you put in is {x}")
}

fn five() -> u32 {
    5
}

fn sum(x: u32, y: u32) -> u32 {
    // or you can use return x + y to forcibly return a value
    x + y
}
