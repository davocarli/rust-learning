use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("Calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher<T, U, V> 
where T: Fn(U) -> V,
{
    closure: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where T: Fn(U) -> V,
{

}

// At this point I made the indpendent decision to allow this to cache multiple values
// impl<T> Cacher<T, U>
// where T: Fn(u32) -> u32,
// {
//     fn new(closure: T) -> Cacher<T, U> {
//         Cacher {
//             closure,
//             values: HashMap::new(),
//         }
//     }

//     fn value(&mut self, arg: u32) -> &u32 {
//         match self.values.get(&arg) {
//             Some(v) => &v,
//             None => {
//                 let v = (self.closure)(arg);
//                 self.values.insert(arg, v);
//                 &self.values.get(&arg).unwrap()
//             }
//         }
//     }
// }

fn generate_workout(intensity: u32, random_number: u32) {

    let mut cacher = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else if random_number == 3 {
        println!("Takea break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", cacher.value(intensity));
    }


    // Personal curiosity
    // let closure = |x| x;

    // let a = closure(String::from("hello"));
    // let b = closure(5);  // Compiler complains because String type was inferred.

    // if true {
    //     let a = closure(String::from("hello"));
    // } else {
    //     let b = closure(5); // Still complains, even though only one will be used.
    // }
    


    // if random_number == 3 && intensity >= 25 {
    //     println!("Take a break today! Remember to stay hydrated!");
    //     return
    // }

    // let calculated_intensity = simulated_expensive_calculation(intensity);

    // if intensity < 25 {
    //     println!("Today, do {} pushups", calculated_intensity);
    //     println!("Next, do {} situps!", calculated_intensity);
    // } else {
    //     println!("Today, run for {} minutes!", calculated_intensity);
    // }
}
