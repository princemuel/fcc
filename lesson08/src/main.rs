use std::collections::HashMap;
use std::io::Bytes;
use unicode_segmentation::UnicodeSegmentation;

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

    enum CellValue {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = [
        CellValue::Int(3),
        CellValue::Text(String::from("blue")),
        CellValue::Float(10.12),
    ];

    match &row[1] {
        CellValue::Int(value) => println!("{}", value),
        _ => println!("Invalid Integer!"),
    }

    // Strings are stored as a collection of UTF-8 encoded bytes
    let _s1 = String::new();
    let s2 = "initial contents";
    let _s3 = s2.to_string();
    let _s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    // let s3 = s1 + &s2;

    let hello = String::from("Hello");
    // let c = hello[0];
    for byte in hello.bytes() {
        println!("{byte}");
    }
    for char in hello.chars() {
        println!("{char}");
    }
    for grapheme in hello.graphemes(true) {
        println!("{grapheme}");
    }

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // overwrites previous

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
