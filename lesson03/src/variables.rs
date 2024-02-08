// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

fn variables() {
    let mut x: u32 = 1;

    {
        let mut x = x;

        x += 2;

        println!("{x}");
    }

    println!("{x}");
}
