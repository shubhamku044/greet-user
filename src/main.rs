use std::io;
use std::io::Write;

fn main() {
    let mut user_input = String::new();
    print!("Enter your name here: ");

    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut user_input)
        .expect("you did't provided me any name");

    if user_input.trim().len() > 0 {
        println!("Hello! {}", user_input)
    } else {
        println!("Hello! world")
    }
}
