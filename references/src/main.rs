fn main() {
    // create a string
    let s1 = String::from("hello");

    // pass a reference to s1
    let len = calculate_length(&s1);

    // s1 is still valid after passing it's reference
    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");

    change(&mut s2);

    println!("{s2}")
}

fn calculate_length(s: &String) -> usize {
    // s goes out of scope
    // but because it is a reference,
    // that original data is not deallocated
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
