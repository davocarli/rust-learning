use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    scores.insert(String::from("Red"), 15);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", scores);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

// fn print_hashmap(map: &HashMap<String, i32>) {
//     println!("{{");
//     for (key, value) in map {
//         println!("    {}: {}", key, value);
//     }
//     println!("}}");
// }
