enum Decision {
    Yes,
    No,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

fn main() {
    // enum Option<T> {
    //     Some(T),
    //     None,
    // };

    let some_number = Some(5);
    let some_string = Some("My String!");

    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);

    println!("{}", sum);

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    
    // match is exhaustive
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // If only checking one condition, you could use if let
    if let Some(3) = some_value {
        println!("three");
    }




    let affirmative_decision: Option<Decision> = Some(Decision::Yes);
    print_decision(affirmative_decision);

    let negative_decision: Option<Decision> = Some(Decision::No);
    print_decision(negative_decision);

    let unsure: Option<Decision> = None;
    print_decision(unsure);
}

fn print_decision(decision: Option<Decision>) {
    match decision {
        None => println!("Decision hasn't been made yet."),
        Some(d) => match d {
            Decision::Yes => println!("They've decided to move forward with the proposal."),
            Decision::No => println!("They've rejected the proposal."),
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}