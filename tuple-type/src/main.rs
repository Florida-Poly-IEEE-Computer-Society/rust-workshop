fn main() {
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The tuple's values: {}, {}, {}", tup.0, tup.1, tup.2);

    // Tuple unpacking
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
