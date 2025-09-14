use parse::parse;
use solve_postfix::solve_postfix;

mod parse;
mod shared;
mod shunting;
mod solve_postfix;
fn main() {
    // Read in input.
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    println!("Welcome to the Rust Workshop Calculator Command Line Tool!\nPlease input a value:");
    while stdin.read_line(&mut buffer).is_ok() {
        let trimmed = buffer.trim_end();
        let parsed = parse(trimmed);
        let postfix = shunting::shunting_algo(parsed);
        let solution = solve_postfix(postfix);
        println!("{solution}");
        println!("Please input a value:");
        buffer = "".to_string();
    }
}
