// Pasted from ch5
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum CanHold {
    Yes,
    Rotate,
    No,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn can_hold_or_rotate(&self, other: &Rectangle) -> CanHold {
        if self.width > other.width && self.height > other.height {
            return CanHold::Yes;
        } else if self.height > other.width && self.width > other.height {
            return CanHold::Rotate;
        }
        CanHold::No
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than 1, got {}", value);
        } else if value > 100 {
            panic!("Guess must be smaller than 100, got {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn failing_test() {
    //     panic!("The test failed!");
    // }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {width: 8, height: 7,};
        let smaller = Rectangle {width: 5, height: 1,};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn larger_can_hold_rotated_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 4,
        };

        let smaller = Rectangle {
            width: 3,
            height: 6,
        };

        match larger.can_hold_or_rotate(&smaller) {
            CanHold::Rotate => assert!(true),
            _ => assert!(false),
        }
    }
    
    #[test]
    fn smaller_cannot_hold_or_rotate_larger() {
        let larger = Rectangle {width: 8, height: 7,};
        let smaller = Rectangle {width: 8, height: 6,};

        match smaller.can_hold_or_rotate(&larger) {
            CanHold::No => assert!(true),
            _ => assert!(false),
        }
    }

    fn larger_rotated_smaller_eq() {
        let larger = Rectangle {width: 8, height: 7,};
        let smaller = Rectangle {width: 8, height: 6,};
        assert_eq!(CanHold::Rotate, larger.can_hold_or_rotate(&smaller));
    }

    #[test]
    fn it_adds_two() {
        let num = 2;
        assert_eq!(4, add_two(num));
    }

    #[test]
    fn it_doesnt_add_three() {
        let num = 2;
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("DCA");
        assert!(
            result.contains("DCA"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess must be smaller than 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String>
}
