#[derive(Debug)]
enum IpAddrKind {
    // IP address enumeration
    V4,
    V6,
}

// We can accept the enumeration, even if it is a different type
fn search_ip(ip: &IpAddrKind) {
    println!("{ip:?}");
}

// each variant can have its own, unique data
#[derive(Debug)]
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // struct like fields
    Write(String),              // single string
    ChangeColor(i32, i32, i32), // includes 3 i32
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let ip4 = IpAddrKind::V4;
    search_ip(&ip4);
    let ip6 = IpAddrKind::V6;
    search_ip(&ip6);

    let message = Message::Move { x: 54, y: 6 };
    message.call();

    // Create an option
    let option_string = Option::Some(String::from("option"));
    // Either retrieve the value if it is Some
    // of if there is a None, end the program
    let string = option_string.unwrap();
}
