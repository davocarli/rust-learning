use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Highlight<'a> {
    location: &'a str,
}

impl<'a> Highlight<'a> {
    fn return_location(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.location
    }
}

fn main() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x; 
    // }

    // println!("r: {}", r);

    // let x = 5;

    // let r = &x;

    // println!("r: {}", r);
    
    // Valid - lifetimes are equal
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    // Valid - shorter lifetime is inside print scope
    let string1 = String::from("abcd");
    
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Invalid - shorter lifetime ends before println!
    // let result;
    // let string1 = String::from("abcd");
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Once upon a time... The End...");
    let first_phrase = novel.split('.').next().expect("Could not find a phrose.");
    let i = Highlight {
        location: first_phrase,
    };

    let s: &'static str = "I have a static lifetime.";
}

// i32          // A type
// &i32         // A reference
// &'a i32      // A reference with an explicit lifetime
// &'a mut i32  // A mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 1. Each parameter that is a reference gets its own lifetime parameter.

// 2. If there is exactly one input lifetime parameter, that lifetime is
//    assigned to all output lifetime parameters.

// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self the lifetime of self is assigned to all output
//    lifetime parameters.