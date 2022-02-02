fn main() {
    println!("Basic loop");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("Counter is {}", counter);

        if counter == 10 {
            break counter;
        }
    };

    println!("The final result is {}", result);
    println!();

    println!("While loop");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!");
    println!();

    println!("For Loop");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
    println!();
    for number in 1..4 {
        println!("{}!", number);
    }

    // Line comment

    /* 
        Block comment
    */
}
