struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Hello {id: i32 },
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    // Splitting RGBColor and ChangeColor makes no sense but is 
    // done here for the sake of practice/example/notes
    RGBColor(i32, i32, i32),
    ChangeColor(Color),
}

fn main() {
    // Matching to literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Other"),
    }

    // Matching to named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // New scope, so y shadows the y outside the match block
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // Matching multiple patterns
    let x = 1;

    match x {
        // One OR two
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching ranges of values
    // Only works with numbers and characters

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("a through j"),
        'k'..='z' => println!("k through z"),
        _ => println!("another character"),
    }

    let p = Point { x: 1, y: 2 };

    // Create two variables, a and b, that are mapped 
    // to the values in the Point struct p
    let Point { x: a, y: b} = p;
    assert_eq!(1, a);
    assert_eq!(2, b);

    // The same pattern as above, but defining x & y
    // which are the names of the fields inside the struct
    let Point {x, y} = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    match p {
        Point {x, y: 0} => {
            println!("y has to be 0, x is {}", x);
        },
        Point {x: 0, y} => {
            println!("x has to be 0, y is {}", y);
        },
        Point {x, y} => {
            println!("The point is {}, {}", x, y);
        },
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit")
        },
        Message::Move { x, y } => {
            println!("Move to {}, {}", x, y)
        },
        Message::Write(text) => {
            println!("Text message: {}", text)
        },
        // Splitting RGBColor and ChangeColor makes no sense but is 
        // done here for the sake of practice/example/notes
        Message::RGBColor(r, g, b) => {
            println!("R: {}, G: {}, B: {}", r, g, b)
        },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("R: {}, G: {}, B: {}", r, g, b)
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("H: {}, S: {}, V: {}", h, s, v)
        },
        _ => println!("Other"),
    }

    // Destructuring can get pretty complex
    let ((feet, inches), Point { x, y }) 
        = ((3, 10), Point { x: 3, y: -3 });

    foo(3, 4);

    let mut current_value = Some(5);
    let new_value = Some(10);

    match (current_value, new_value) {
        (Some(_), Some(_)) => {
            println!("Can't change existing value");
        },
        (Some(_), None) => {
            println!("The value has already been set");
        },
        _ => {
            current_value = new_value;
        }
    }

    println!("Value is {:?}", current_value);

    let numbers = (1, 2, 3, 4, 5);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("First = {}, third = {}, fifth = {}", first, third, fifth)
        }
    }

    // Prefixing variable name with underscore indicates it won't be used
    // but the value is still bound to the variable.
    let _x = 5;
    let y = 10;

    // Match first & last numbers in tuple
    let tup = (1, 2, 3, 4, 5);

    match tup {
        (first, .., last) => println!("First and last are {}, {}", first, last),
    }

    // Match with conditionals
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // Match with assigning variable
    let msg = Message::Hello { id: 5};

    match msg {
        Message::Hello { 
            id: id_variable @ 3..=7, // @ operator assigns value to id_variable
        } => println!("Found an id in range: {}", id_variable),
        _ => println!("Other"),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
