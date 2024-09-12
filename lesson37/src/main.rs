use std::fmt;
use std::ops::Add;

fn main() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    HomoSapiens::fly();

    <HomoSapiens as Witch>::fly();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("w = {w}");

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Thunk = Box::new(|| println!("hi"));

    // fn bar() -> ! {}

    let list_of_nums = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_nums.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> =
        list_of_nums.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }
    fn closures(a: i32) -> Box<dyn Fn(i32) -> i32> {
        if a > 0 {
            Box::new(move |b| a + b)
        } else {
            Box::new(move |b| a - b)
        }
    }
}

fn generic<T: ?Sized>(t: &T) {}

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }
// }

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("up, up and away...");
    }
}

struct HomoSapiens;

trait Aeronaut {
    fn fly();
}
trait Witch {
    fn fly();
}

impl HomoSapiens {
    fn fly() {
        println!("*waving arms furiously*");
    }
}
impl Aeronaut for HomoSapiens {
    fn fly() {
        println!("This is your captain speaking");
    }
}

impl Witch for HomoSapiens {
    fn fly() {
        println!("up, up and away...");
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_two_times<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_points() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }
}
