fn main() {
    let x = 5;
    let y = 6;
    let num = function_that_returns_value(5, 6);
    println!("The sum of the values is {num}");
}

fn my_first_function() {
    println!("Hello. I am your function. YIPPIE!!!");
}

fn function_with_parameters(x: u32) {
    println!("The number that you put in is {x}")
}

fn function_that_returns_value(x: u32, y: u32) -> u32 {
    // return the sum of x and y
    return x + y;
}
