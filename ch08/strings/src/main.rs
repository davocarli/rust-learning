use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    s.push_str(s2);

    let s1 = s1 + &s2;
    let s1 = format!("{}{}", s1, s3);

    println!("{}", s1);

    let string = "hello";

    // List of bytes
    println!("bytes");
    for b in string.bytes() {
        println!("{}", b);
    }

    // List of chars
    for c in string.chars() {
        println!("{}", c);
    }

    // List of graphemes
    for g in string.graphemes(true) {
        println!("{}", g);
    }
}
