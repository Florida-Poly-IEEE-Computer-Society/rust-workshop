fn main() {
    // // create a variable x
    // let mut s = String::from("Hello World");
    // // create 2 references and 1 mutable reference
    // let sr1 = &s;
    // let sr2 = &s;
    // let mut msr3 = &mut s;
    // // Lets print the original string they reference
    // println!("{sr1} and {sr2} and {msr3}");

    let mut s = String::from("Hello World");
    // create 2 references
    let sr1 = &s;
    let sr2 = &s;
    // Lets print the original string they reference
    println!("{sr1} and {sr2}");

    // create a mutable reference
    let mut msr3 = &mut s;
    // sr1 and sr2 are never used again, we are fine
    println!("{msr3}");
}

fn dangling() -> &String {
    let str = String::from("Perfectly fine string");

    &str
}
