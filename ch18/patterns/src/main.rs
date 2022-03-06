use std::fmt::Display;

fn main() {
    #[derive(Debug)]
    enum Language {
        // Languages I know
        English,
        Spanish,
        Romanian,
        Italian,
        // Languages I don't know
        Japanese,
        Russian,
        German,
    }

    let language = Language::Romanian;

    if let Language::English = language {
        println!("Default language!");
    }

    // Match is exhaustive
    match language {
        Language::English => println!("Hello World!"),
        Language::Spanish => println!("Hola Mundo!"),
        Language::Romanian => println!("Bună Lume!"),
        Language::Italian => println!("Ciao Mondo!"),
        lang => println!("Unsupported language! {:?}", lang),
    }

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    // If let is not exhaustive
    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: basic");
    }

    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    // enumerate gives you index in addition to value - super useful
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let is technically let PATTERN = EXPRESSION;

    let x = 5;
    let (x, y, z) = (1, 2, 3);
    let (first_name, _,  age) = ("David", "Carli-Arnold", 25);

    // Irrefutable patterns always match
    let x = 5;

    // Refutable patterns might not match
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    };

    // Function parameters, let statements (not if let), and for loops
    // can only accept irrefutable patterns. These must always match.

    // if let can accept refutable patterns

    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
