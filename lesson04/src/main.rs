// fn main() {
//     let a = [5; 10];
//     let mut sum = 0;
//     for x in a {
//         sum += x;
//     }
//     println!("{}", sum);
// }
fn main() {
    let first = String::from("Ferris");

    let mut x = "Hello";
    let full = add_suffix(first);
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
