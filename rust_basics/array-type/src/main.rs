fn main() {
    // Integer array
    let array = [1, 2, 3, 4, 5];
    println!("The first element in array is {}", array[0]);

    // Initialize an array of same value
    let same_value_array = [3; 5];

    // Days of the week
    let days_of_the_week = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    println!("{:?}", same_value_array);
    println!("{:?}", days_of_the_week);
}
