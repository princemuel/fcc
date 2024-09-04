struct Point {}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest_01(&number_list);
    println!("The largest number is: {largest}");

    let number_list = vec![100, 34, 6000, 89, 54, 2, 43, 8];
    let largest = get_largest_01(&number_list);
    println!("The largest number is: {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest_02(&char_list);
    println!("The largest char is: {largest}");
}

fn get_largest_02<T: PartialOrd + Copy>(number_list: &[T]) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > &largest {
            largest = *number;
        }
    }
    largest
}

fn get_largest_01(number_list: &[i32]) -> i32 {
    let mut largest = number_list[0];

    for number in number_list {
        if number > &largest {
            largest = *number;
        }
    }
    largest
}
