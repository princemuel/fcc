use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess...");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        // using the 'turbofish' syntax
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Bah! It's too small!".red()),
            Ordering::Greater => println!("{}", "Urgh! It's too big!".red()),
            Ordering::Equal => {
                println!("{}", "Hoorah! You win!".green());
                break;
            }
        };
    }
}
