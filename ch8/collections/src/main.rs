fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![1, 2, 3, 4, 5];

    v2.push(6);

    let element = &v2[2];
    println!("The element is {element}");

    match v.get(20) {
        Some(element) => println!("The element is {element}"),
        None => println!("There is no element at that index"),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    
}
