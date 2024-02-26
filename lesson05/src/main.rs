#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let width = 30;
    let height = 50;
    let rect = (width, height);
    let rectangle = Rectangle { width, height };

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

    println!("Rect: {:#?}", rectangle);
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
