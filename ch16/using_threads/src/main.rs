use std::{thread, time::Duration};

fn main() {
    // This thread will not finish. When main thread ends
    // the spawned thread will stop.
    thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread printing {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let sum_items = thread::spawn(|| -> i32 {
        let mut total: i32 = 0;
        for i in 1..10 {
            println!("Handled thread printing {}", i);
            total += i;
            thread::sleep(Duration::from_secs(1));
        }
        total
    });

    for i in 1..5 {
        println!("Main thread printing {}", i);
        thread::sleep(Duration::from_secs(1));
    };

    let result = sum_items.join().unwrap();

    println!("The result is {}", result);

    let v = vec![1, 2, 3, 4, 5];

    let print_vector = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    print_vector.join().unwrap();
}
