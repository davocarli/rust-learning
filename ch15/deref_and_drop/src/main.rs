use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}


fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str

    // Rust does deref coercion when it finds types and trait
    // implementations in three cases:
    // - From &T to &U when T: Deref<Target=U>
    // - From &mut T to &mut U when T: DerefMut<Target=U>
    // - From &mut T to &U when T: Deref<Target=U>

    // tl;dr cannot do coercion going from immutable to mutable


    let data1 = CustomSmartPointer { data: String::from("my data") };
    let data2 = CustomSmartPointer { data: String::from("more data") };

    drop(data1);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}