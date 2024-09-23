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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // use &rect2 not rect2 because you want to borrow ownership not transfer
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // transferring ownership will render rect2 and 3 unusable after the line
}

struct Rectangle {
    // remember in rust you can do declaration before/after using reference
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

    fn square(size: u32) -> Self {
        // call this using :: instead of .
        Self {
            width: size,
            height: size,
        }
    }
}
