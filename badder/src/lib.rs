#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(value: i32) -> i32 {
    value + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}
// pub fn greeting(_name: &str) -> String {
//     format!("Hello")
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        // if value < 1 || value > 100 {
        // if !(1..=100).contains(&value) {
        //     panic!("Your guess should be between 1 and 100...Got `{value}`");
        // }
        if value < 1 {
            panic!(
                "Your guess must be greater than or equal to 1...Got `{value}`"
            );
        } else if value > 100 {
            panic!(
                "Your guess must be less than or equal to 100...Got `{value}`"
            );
        }

        Self { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 1 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal 4"))
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, got `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic(expected = "Your guess must be less than or equal to 100")]
    fn greater_than_100_should_panic() {
        Guess::new(-2);
    }
}
