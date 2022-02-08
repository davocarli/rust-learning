fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        println!();
        println!("{:?}", cell);
        match cell {
            SpreadsheetCell::Int(i) => println!("{i}"),
            _ => println!("Not an int"),
        }
    }

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{i}"),
        _ => println!("Not an int"),
    }

    
}
