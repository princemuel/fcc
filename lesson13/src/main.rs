use std::thread::sleep;
use std::time::Duration;

//
fn main() {
    let intensity = 10;
    let random_num = 7;
    generate_workout(intensity, random_num);

    let x = 4;
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    // let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);

    let equal_to_x = |z| z == x;
    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

pub fn expensive_calculation(intensity: u16) -> u16 {
    println!("calculating slowly...");
    sleep(Duration::from_secs(3));
    intensity
}

struct Memoizer<T>
where
    T: Fn(u16) -> u16,
{
    calculation: T,
    result: Option<u16>,
}

impl<T> Memoizer<T>
where
    T: Fn(u16) -> u16,
{
    pub fn new(calculation: T) -> Self {
        Self { calculation, result: None }
    }

    fn value(&mut self, arg: u16) -> u16 {
        match self.result {
            Some(value) => value,
            None => {
                let value = (self.calculation)(arg);
                self.result = Some(value);
                value
            },
        }
    }
}

pub fn generate_workout(intensity: u16, random_num: u16) {
    // let expensive_result = expensive_calculation(intensity);

    let mut cached_result = Memoizer::new(|value| {
        println!("calculating slowly...");
        sleep(Duration::from_secs(3));
        value
    });

    if intensity < 25 {
        println!("Today, go {} pushups", cached_result.value(intensity));
        println!("Next, do {} situps", cached_result.value(intensity));
    } else if random_num == 3 {
        println!("Take a break today. Remember to stay hydrated");
    } else {
        println!("Today, run for {} minutes", cached_result.value(intensity));
    }
}
