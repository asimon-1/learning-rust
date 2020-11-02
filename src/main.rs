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
        self.area() > other.area() && self.height > other.height && self.width > other.width
    }
}

#[derive(Debug)]
struct Circle {
    radius: u32,
}

impl Circle {
    fn area(&self) -> f32 {
        3.14159 * (self.radius as f32).powf(2.0)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 2,
        height: 51,
    };

    let circ1 = Circle { radius: 5 };

    println!(
        "The area of the rectangel is {} square pixels.",
        rect1.area()
    );
    println!("The area of the circle is {} square pixels.", circ1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
