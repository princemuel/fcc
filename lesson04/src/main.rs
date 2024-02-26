// fn main() {
//     let a = [5; 10];
//     let mut sum = 0;
//     for x in a {
//         sum += x;
//     }
//     println!("{}", sum);
// }
// fn main() {
//     let first = String::from("Ferris");

//     let mut _x = "Hello";
//     let full = add_suffix(first);
//     println!("{full}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

// fn main() {
//     // ..... Ownership Rules
//     // 1. Each value in Rust has a variable that's called it's owner
//     // 2. There can only be one owner at a time
//     // 3. When the owner goes out of scope, the value will be dropped
//     let x = "hello";
//     let y = x; // Copy: implemented on &str, bool, (u|i|f)*, char

//     let s1 = String::from("hello");
//     // let s2 = s1; // Move (not shallow copy) the value of s1 is moved into s2, s1 is then invalidated
//     let s2 = s1.clone(); // just clone, without moving

//     println!("{}", x);
//     println!("{}", y);
//     println!("{}", s1);
//     println!("{}", s2);
// }

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s3 = String::from("hello");
    change(&mut s3);

    let mut s = String::from("hello world");
    let s2 = "hello world";

    // let hello = &s[..5]; //  &s[0..5];
    // let world = &s[6..]; // &s[6..11]; to slice whole string, &s[..]

    // let word = first_word(&s);
    let word = first_word(&s);
    // s.clear(); // can't do this =>  println!("The first word is: {}", word); after this line

    let a = [1, 2, 3, 4, 5];
    let sliced_array = &a[0..2];
    println!("The first word is: {}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..idx];
        }
    }

    &s[..]
}
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (idx, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return idx;
//         }
//     }

//     s.len()
// }
