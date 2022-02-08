struct Point {
    x: i32,
    y: i32,
}

struct GenericPoint<T> {
    x: T,
    y: T,
}

struct MismatchedPoint<T, U> {
    x: T,
    y: U,
}

impl<T> GenericPoint<T>{
    fn x(&self) -> &T {
        &self.x
    }
}

impl GenericPoint<f64> {
    fn y(&self) -> f64{
        self.y
    }
}

impl<T, U> MismatchedPoint<T, U> {
    fn mixup<V, W>(self, other: MismatchedPoint<V, W>) -> MismatchedPoint<T, W> {
        MismatchedPoint { x: self.x, y: other.y }
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['d', 'a', 'v', 'i', 'd'];

    let largest = get_largest(char_list);

    println!("The largest char is {}", largest);

    let p1 = Point {x: 5, y: 10};
    let p2 = GenericPoint {x: 5.0, y: 10.5};
    let p3 = MismatchedPoint { x: 5, y: 10}; // Can be same type
    let p3 = MismatchedPoint {x: 5, y: 10.5}; // Can be diff types

    let p = GenericPoint{ x: 5, y: 10};
    p.x();
    // p.y(); // Not available

    let p = GenericPoint { x: 5.0, y: 10.0 };
    p.x();
    p.y(); // Available because they're f64


    let p1 = MismatchedPoint {x: 5, y: 10.4 };
    let p2 = MismatchedPoint {x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}

fn get_largest_i32(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
