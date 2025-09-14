fn main() {
    let x = loop_loop_function();
    println!("The value from the loop is: {x}");
}

fn conditional_functions(x: u32) {
    // conditional that takes an integer x and prints a message depending on its value
    if x > 5 {
        println!("x is above 5");
    } else if x < 3 {
        println!("x is below 3");
    } else {
        println!("x is somewhere between 3 and 5");
    }
}

fn loop_loop_function() -> i32 {
    let mut loop_control = 0;
    let large_value = loop {
        // prints out 5 times
        println!("loop_control's value is: {loop_control}");
        loop_control += 1;
        if loop_control > 5 {
            // break stops the loop from running
            // break followed by a value will bind that value to outer block
            break loop_control * 2;
        }
    };
    // returns the calculated value
    large_value
}

fn while_loop_function(y: u32) {
    let mut y = y;
    while y > 0 {
        println!("Count down in {y}");
        y -= 1;
    }
}

fn for_loop_function(my_vector: Vec<u32>) {
    for index, i in my_vector.iter().enumerate() {
        print!("")
    }
}
