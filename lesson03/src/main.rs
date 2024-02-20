use std::{thread, time};
fn main() {
    let condition = true;
    let _number = if condition { 5 } else { 6 };

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coin = Coin::Penny;
    println!("Value of coin: {}", value_in_cents(coin));

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut counter = 3;
    while counter != 0 {
        println!("{}!", counter);

        counter -= 1;
        thread::sleep(time::Duration::from_secs(1));
    }

    println!("Lift OFF!!!");

    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("the value is {}", element);
    }

    let s = "hello world";
    for char in s.chars() {
        println!("the value is {}", char);
    }

    for num in 1..4 {
        println!("the value is {}", num);
    }

    println!("GO!!!");

    fizzbuzz(100);
}

fn fizzbuzz(end: u32) {
    for number in 1..=end {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz")
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}
