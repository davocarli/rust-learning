mod front_of_house;

pub use crate::front_of_house::hosting;
use self::front_of_house::hosting as relative_hosting;
use self::front_of_house::hosting::add_to_waitlist as waitlist; // Bad practice - bring parent module
// use rand::Rng;
use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io::{self, Write};
use std::io::*;

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // with use
    hosting::add_to_waitlist();

    // with relative path and rename
    relative_hosting::add_to_waitlist();

    // Bad practice
    waitlist();
}


fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches") 
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_breakfast() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
}

pub fn eat_appetizer() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
