fn main() {
    let s1 = String::from("hello");


    // Method One - Passing back the variable
    let (mut s1, len) = calculate_length_1(s1);
    println!("Method one \"{}\" is of length {}", s1, len);

    // Method Two - Using a reference to the variable with &
    let len = calculate_length_2(&s1);
    println!("Method two \"{}\" is of length {}", s1, len);

    // Method Three - Allows changing of string by making it mutable
    let len = calculate_length_3(&mut s1);
    println!("Method three \"{}\" is of length {}", s1, len);

}

// Method One - Passing back the variable
fn calculate_length_1(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// Method Two - Using a reference to the variable with &
fn calculate_length_2(s: &String) -> usize {
    let length = s.len();
    length
}

// Method Three - Allows changing of string
fn calculate_length_3(s: &mut String) -> usize {
    s.push_str(" world!");
    let length = s.len();
    length
}

/* -- The Rules of References
1. At any given time, you can have either one mutable reference
   or any number of immutable references.

2. References must always be valid.
*/