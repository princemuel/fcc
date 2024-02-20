// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true")
//     } else {
//         println!("condition was false")
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4")
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3")
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2")
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// // }
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The vakue of number is: {number}")
// }

// fn main() {
//     loop {
//         println!("again");
//     }
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}")
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}")
// }

// fn main() {
//     let mut count = 3;

//     while count != 0 {
//         println!("{count}!");
//         count -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is {}", a[index]);

//         index += 1;
//     }
// }
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < a.len() {
//         println!("the value is {}", a[index]);

//         index +=
//     }
// }
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is {element}");
//     }
// }
// fn main() {
//     for number in (1..4).rev() {
//         println!("the value is {number}");
//     }
//     println!("LIFTOFF!!!");
// }

fn control_flow() {
    let mut x = 0;

    'a: loop {
        x += 1;

        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }

        break;
    }
}
