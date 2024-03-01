// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//         fn serve_order() {}
//     }
//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }
// use std::fmt;
// use std::io;
// fn fmt_result() -> fmt::Result {}
// fn io_result() -> io::Result<()> {}

use std::fmt::Result;
use std::io::Result as IOResult;

fn fmt_result() -> Result {
    Ok(())
}
fn io_result() -> IOResult<()> {
    Ok(())
}

fn serve_order() {}

mod front_of_house;
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
use core::fmt;

// use self::front_of_house::hosting;
// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;
use rand::{CryptoRng, Rng};
use std::alloc::*;
use std::io::{self, Write};

pub fn eat_at_retaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1..100);

    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheat");

    let order_01 = back_of_house::Appetizer::Soup;
    let order_02 = back_of_house::Appetizer::Salad;
}
