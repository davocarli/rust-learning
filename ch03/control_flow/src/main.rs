use std::io;
use std::process;

fn main() {
    // Control Flow

    let condition = true;
    let number = if condition { 5 } else { 15 };

    println!("Please enter a number: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a number");

    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => process::exit(1),
    };

    if number < 10 {
        println!("The number is less than 10");
    } else if number < 22 {
        println!("The number is more than 9, but less than 22");
    } else {
        println!("The number is bigger than 21");
    }
}
