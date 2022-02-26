// Making my own iterators
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Here I build a struct that holds a vec of i32's, and will iterate through them
// It also holds another vec of 'cursed numbers' - if we encounter one of these it
// will be skipped
struct NumberKeeper {
    count: u32,
    pub numbers: Vec<i32>,
    pub cursed: Vec<i32>,
}

impl NumberKeeper {
    fn new(numbers: Option<Vec<i32>>, cursed: Option<Vec<i32>>) -> NumberKeeper {
        let nums: Vec<i32>;
        let curse: Vec<i32>;

        match numbers {
            Some(item) => nums = item,
            _ => nums = vec![],
        }

        match cursed {
            Some(item) => curse = item,
            _ => curse = vec![],
        }

        NumberKeeper {count: 0, numbers: nums, cursed: curse}
    }
}

impl Iterator for NumberKeeper {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
            None
    }
}

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // immutable refs
    // let mut v1_iter = v1.iter_mut(); // mutable refs
    // let mut v1_iter = v1.into_iter(); // owned types

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x|x+1).collect();

    assert_eq!(v1, vec![1, 2, 3]);
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn test_shoes() {
    let shoe_size: u32 = 45;

    let mut shoes: Vec<Shoe> = vec![];

    let styles = vec!["Running", "Sneakers", "Chelsea Boot", "Loafers"];
    
    for style in styles.into_iter() {
        for size in 38..47 {
            shoes.push(
                Shoe { size, style: String::from(style) }
            )
        }
    }

    let my_size = shoes_in_my_size(shoes, shoe_size);

    assert_eq!(my_size, vec![
        Shoe{size: 45, style: String::from("Running")},
        Shoe{size: 45, style: String::from("Sneakers")},
        Shoe{size: 45, style: String::from("Chelsea Boot")},
        Shoe{size: 45, style: String::from("Loafers")}
         ]);
}
