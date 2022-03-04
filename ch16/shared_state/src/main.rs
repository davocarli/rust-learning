use std::sync::{Arc, Mutex};
use std::thread;

// Arc is like Rc but thread safe
// Can be used across multiple threads

// Arc::new(Mutex::new())) is the multithreaded equivalent of 
// Rc::new(RefCell::new()))
fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));

    let mut threads = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handled_thread = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        threads.push(handled_thread);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    println!("Final result is {:?}", counter);
}
