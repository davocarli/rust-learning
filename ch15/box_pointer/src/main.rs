enum List<a> {
    Cons(a, Box<List<a>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));



    let b = Box::new(5);
    println!("b = {}", b);

    // Use Box when:
        // - Have a type whose exact size can't be known at compile time and want to use
        //   the value in a context that requires knowing the size
        // - When you have a large amount of data and want to transfer ownership of the
        //   data without copying
        // - When you own a value and only care that it implements a certain trait, rather
        //   than the type of the value/object
}
