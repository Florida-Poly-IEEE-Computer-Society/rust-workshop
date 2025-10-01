struct User {
    // template for a User struct
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        // fill in values for that type
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User1's username is {}", user1.username);
    user1.active = false;
    if user1.active {
        println!("User1 is active");
    } else {
        println!("User1 is inactive");
    }
}
