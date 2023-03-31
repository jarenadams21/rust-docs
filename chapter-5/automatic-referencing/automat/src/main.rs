#[derive(Debug)]
struct Rectangle {

    width: u32,
    height: u32
}

// Associated functions
impl Rectangle {

    // Similar to new
    // Associated functions that aren’t methods 
    // are often used for constructors that will
    // return a new instance of the struct.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn isRectBig(rect: &Rectangle) -> bool {

        if (rect.area() > 25) {
            { true }
        }
        else 
            { false }
    }

    fn area(&self) -> u32 {

        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {

        if (self.width > rect2.width && self.height > rect2.height)
           { true }
        else
           { false }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle {
        width: 3,
        height: 4,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Is rect1 big? {}", Rectangle::isRectBig(&rect1));
    println!("Is rect4 big? {}", Rectangle::isRectBig(&rect4));
}