#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, w: u32) {
        self.width = w;
    }
    // Struct methods can match variable names, 
    // Rust distinguishes between methods and vars
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The width of the rectangle is {} square pixels.",
        rect1.width
    );
    rect1.set_width(40);
    println!(
        "The width of the rectangle is {} square pixels.",
        rect1.width
    );
}