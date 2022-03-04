use std::sync::mpsc;
use std::{thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    // Multiple senders for one receiver
    let tx2 = tx.clone();

    thread::spawn(move || {
        let msg = String::from("Hello message!");
        // Should handle error in prod, not just unwrap
        tx.send(msg).unwrap();
        // Can't use a variable after it's sent
        // println!("msg is {}", msg);
    });

    // Waits for a message to be received
    let received = rx.recv().unwrap();

    // Immediately returns a result. Ok if there is a value available 
    // let received = rx.try_recv().unwrap();

    println!("Message received! {}", received);

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("there,"),
            String::from("how"),
            String::from("are"),
            String::from("you?"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Message received! {}", received);
    }
}
