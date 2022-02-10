fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works2() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // A test that takes a long time to run
        // Can run with "cargo test -- --ignored"
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_add(2, 2));
    }
}

// Alternative organization that I think is more intuitive

fn exponential(num: i32, exp: i32) -> i32 {
    let mut result = num;
    for i in 1..exp {
        println!("{}, {}", result, i);
        result *= num;
    }
    result
}

#[cfg(test)]
mod exponential_tests {
    use super::exponential;

    #[test]
    fn square_2() {
        assert_eq!(4, exponential(2, 2));
    }

    #[test]
    fn three_to_fourth() {
        assert_eq!(81, exponential(3, 4));
    }

    #[test]
    fn negative_five_cubed() {
        assert_eq!(-15625, exponential(-25, 3));
    }
}