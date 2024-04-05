fn main() {
    let a = [1, 2, 3];

    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v2 = vec![1, 2, 3];
    }

    let v2 = vec![1, 2, 3, 4, 5];

    let mut v3 = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    // println!("The third element is {}", third);

    match v.get(10) {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element"),
    }

    for idx in &mut v3 {
        *idx += 50;
    }

    for idx in &v3 {
        println!("{}", idx);
    }

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

    match &row[1] {
        SpreadsheetCell::Int(value) => println!("{}", value),
        _ => println!("Invalid Integer!"),
    }

    // Strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    // let s3 = s1 + &s2;

    let hello = String::from("Hello");
    // let c = hello[0];
}
