fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    // we can use Enums to save different data types in the same vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Luis")),
    ];

    for i in &row {
        match *i {
            SpreadsheetCell::Int(value) => println!("SpreadsheetCell is {}", value),
            _ => println!("Values is not an int"),
        }
    }
}
