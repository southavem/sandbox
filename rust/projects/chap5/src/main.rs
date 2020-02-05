fn main() {
    let rectangle = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rectangle)
    );
}

fn area(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}
