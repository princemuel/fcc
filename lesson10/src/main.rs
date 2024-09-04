//

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

#[derive(Debug)]
struct Pointy<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pointy<T, U> {
    fn mixup<V, W>(self, other: Pointy<V, W>) -> Pointy<T, W> {
        Pointy { x: self.x, y: other.y }
    }
}

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

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    println!("p1 = {:?}, p2 = {:?}", p1, p2);

    let p1 = Pointy { x: 5, y: 10.4 };
    let p2 = Pointy { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
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
