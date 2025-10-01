fn main() {
    // {
    //     // create variable x in this scope
    //     // x owns the string "My value"
    //     let x = String::from("Hello world");
    //     // saying that y owns the value of x
    //     let y = x;
    //     // Because x does not own the value
    //     // It is invalid
    //     println!("{x}");
    // }
    // // y is out of scope, you can not reference it after
    // println!("{y}");

    // Fixed code
    let y = {
        let x = String::from("Hello world");
        // clone the data from x
        // y owns that clone
        let y = x.clone();
        println!("{x}");
        // y is the expression for this block
        y
    };
    println!("{y}");
}
