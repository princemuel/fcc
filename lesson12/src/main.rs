use std::fmt::Display;

pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let a = String::from("abcd");
    let b = String::from("xyz");

    let result = longest(a.as_str(), b.as_str());
    println!("The longest string is {}", result);

    {
        let b = String::from("xyz");

        let result = longest(a.as_str(), b.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. some years ago...");
    let first_sentence = novel.split(".").next().expect("could not find...");
    let _i = ImportantExcerpt { part: first_sentence };

    let _s: &'static str = "I have a static lifetime";
}

pub fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Attention please: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// the lifetime of the result will be the same
// as the smallest lifetime of the args
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// cannot return a reference to a function's local variable

// * 1. Each parameter that is a reference gets its own lifetime parameter

// * 2. If there is exactly one input lifetime parameter,
// *    that lifetime is assigned to all output lifetime parameters

// * 3. If there are multiple input lifetime parameters,
// *    but one of them is &self or &mut self, the lifetime of self
// *    is assigned to all output lifetime parameters

pub fn first_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();
    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[0..idx];
        }
    }

    sentence
}
