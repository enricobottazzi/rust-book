fn main() {

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "rectangle is {:?}", rectangle
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    println!(
        "The width of the rectangle is {} square pixels.",
        rectangle.width()
    );

    println!(
        "The width of the rectangle is {} square pixels.",
        rectangle.width
    );

    println!(
        "create a square with size 3: {:?}", Rectangle::square(3)
    );
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}