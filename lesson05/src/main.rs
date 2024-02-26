#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let width = 30;
    let height = 50;
    let rect = (width, height);
    let rectangle = Rectangle { width, height };
    let rect_01 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect_02 = Rectangle {
        width: 40,
        height: 50,
    };

    let square = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        calc_area(rect)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area(&rectangle)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    println!("Rect: {:#?}", rectangle);
    println!("Square: {:#?}", square);
    println!("Rect can hold Rect 01 {}", rectangle.can_hold(&rect_01));
    println!("Rect can hold Rect 02 {}", rectangle.can_hold(&rect_02));
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn calc_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
