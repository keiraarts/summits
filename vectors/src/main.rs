enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut s = String::new();

    let mut row = vec![
        SpreadsheetCell::Int(100),
        SpreadsheetCell::Float(10.2),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There was no third element. :("),
    }

    v.push(100);
    row.push(SpreadsheetCell::Text(String::from("Canada, Ontario")));
}

pub fn iterations() {}
