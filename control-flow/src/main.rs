fn main() {
    let x: bool = 10 > 5;
    println!("{x}");

    let number = 3;

    if number < 5 {
        println!("Number is less than 5");
    }

    if number == 3 {
        println!("The number is 3");
    } else if number > 3 {
        println!("The number is greater than 3");
    } else {
        println!("The number is less than 3");
    }

    let my_number = if number > 3 {
        "greater than three"
    } else {
        "less than or equal to three"
    };

    println!("{my_number}");
}
