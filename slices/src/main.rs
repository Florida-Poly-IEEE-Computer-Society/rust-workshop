fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[1..3];
    println!("{:?}", slice);

    let owned_str = String::from("Hello world");
    let reference = first_word(&owned_str);
    println!("{reference}");
}

// takes a string slice as a parameter
fn first_word(s: &str) -> &str {
    // returns a slice of bytes
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // checks if there is a space
        if item == b' ' {
            // returns the reference early
            return &s[0..i];
        }
    }

    // references the original data, which is still valid outside
    // of the function
    &s[..]
}
