struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // ); error because Rectangle don't implement copy so rect1 is moved at line 14
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}