// Yo, check it – defining the dimensions of a rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// Doing the math to get that area of the rectangle, you feel me?
impl Rectangle {
    fn area(&self) -> u32 {
        // Just multiply them two numbers, simple as that, ya know?
        self.width * self.height
    }
}

fn main() {
    // We creating this lil' rectangle, keepin' it on the down-low
    let small = Rectangle {
        width: 10,
        height: 20,
    };

    // Printin' out the deets and the real deal area
    println!(
        "Aight, so the width is {}\nHeight is {}\nAnd boom, the area is {}",
        small.width,
        small.height,
        small.area()
    );
}

