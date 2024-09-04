use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
}

pub struct Post {
    pub author: String,
    pub headline: String,
    pub content: String,
}
impl Summary for Post {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// pub fn notify(a: &impl Summary, b: &impl Summary)
// pub fn notify(a: &(impl Summary + Display), b: &impl Summary)
// pub fn notify<T: Summary>(a: &T, b: &T)
// pub fn notify<T: Summary + Display>(a: &T, b: &T)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn alert<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U)-> i32
// pub fn some_fn<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// { }

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

fn main() {
    let tweet = Tweet {
        username: String::from("@princemuel"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Prince Muel"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not actually falling!"),
    };
    let post = Post {
        author: String::from("Prince Muel"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not actually falling!"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    println!("Post summary: {}", post.summarize());

    notify(&tweet);
    alert(&tweet);
}
