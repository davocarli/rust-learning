fn main() {
    let sum = my_function(11, 22);
    println!("The sum is {}", sum);
}

// Functions
fn my_function(x: i32, y:i32) -> i32 {
    println!("The value of x, y is {}, {}", x, y);
    x + y
}