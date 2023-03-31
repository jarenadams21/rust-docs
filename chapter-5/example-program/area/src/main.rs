// Custom debug attribute for printing structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // dbg! returns ownership of the expression's value
    dbg!(&rect1);
}

// Struct approach for clearer/labeled data
// relationships and purpose of values more obvious
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Original approach
fn area_original(width: u32, height: u32) -> u32 {
    width * height
}

// Tuple approach
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}