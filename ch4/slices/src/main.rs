fn main() {
    let mut s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(&s2);
    // s.clear();
    println!("The first word is {}", word);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
