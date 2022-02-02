fn main() {
    /* -- Ownership rules --
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    // {} Creates a new scope
    { // s is not valid here, it's not yet declared

        let s = "hello"; // String literal | fixed in size
        let s = String::from("hello"); // String type, variable size

        // do stuff with s, whatever we need
    } // scope ends, so s is dropped

    let x = 5;
    let y = x; // Copy

    // let s1 = String::from("hello");
    // let s2 = s1; // Move - NOT shallow copy (as I expected)

    // println!("{}, world!", s2); // Should print
    // println!("{}, world!", s1); // Should fail because of the move

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s2);
    println!("{}, world!", s1); // Should work this time

    // Simple types implement the "Copy Trait" which allows them to be copied instead of moved

    // takes_ownership(s1);
    // println!("{}", s1); // Can't do this because takes_ownership has taken ownership of variable

    let s1 = returns_ownership(s1);
    println!("{}", s1);

    let x = 5;
    makes_copy(x);
    println!("{}", x); // Can do this because x is copied not moved

}

fn takes_ownership(string: String) { // Strings will be moved
    println!("{}", string);
}

fn makes_copy(integer: i32) { // Simple will be copied
    println!("{}", integer);
}

fn returns_ownership(string: String) -> String { // Playing around with returning value
    println!("{}", string);
    string
}